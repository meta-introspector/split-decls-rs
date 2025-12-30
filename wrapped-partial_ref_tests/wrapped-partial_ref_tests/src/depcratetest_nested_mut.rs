// Generated macro for test_nested_mut (function)
macro_rules! Depcratetest_nested_mut {
() => {
// Module: crate
// Provides: {"test_nested_mut"}
// Dependencies: {}
# [test] fn test_nested_mut () { let mut s = Bar { foo : Foo { a : 1 , b : 2 } , a : 3 , } ; let mut sr = s . into_partial_ref_mut () ; split_borrow ! (foo_a = & (mut PartFoo | PartA) sr) ; std :: mem :: swap (foo_a . part_mut (PartFoo | PartA) , sr . part_mut (PartA)) ; assert_eq ! (s . foo . a , 3) ; assert_eq ! (s . a , 1) ; }
};
}
