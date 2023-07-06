use multi_index_map::MultiIndexMap;

#[derive(MultiIndexMap, Clone)]
struct TestElement1<'a> {
    #[multi_index(ordered_unique)]
    field1: &'a usize,
    field2: usize
}

#[derive(MultiIndexMap, Clone)]
struct TestElement2<'a, 'b, 'c: 'a + 'b> {
    #[multi_index(ordered_unique)]
    field1: &'a usize,
    #[multi_index(ordered_unique)]
    field2: &'b usize,
    field3: &'c usize
}


#[test]
fn test_ref_fields() {
    let mut m1 = MultiIndexTestElement1Map::default();
    let a1 = 37;
    let a2 = 31;

    m1.insert(TestElement1 { field1: &a1, field2: 0});
    m1.insert(TestElement1 { field1: &a2, field2: 1});

    let mut it = m1.iter_by_field1();
    assert_eq!(it.next().unwrap().field2, 1);
    assert_eq!(it.next().unwrap().field2, 0);


    let mut m2 = MultiIndexTestElement2Map::default();
    m2.insert(TestElement2 { field1: &a1, field2: &a2, field3: &0 });
    m2.insert(TestElement2 { field1: &a2, field2: &a1, field3: &1 });

    let mut it = m2.iter_by_field1();
    assert_eq!(it.next().unwrap().field3, &1);
    assert_eq!(it.next().unwrap().field2, &a2);
}
