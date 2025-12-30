// Generated macro for test_nested_mut_to_const (function)
macro_rules! Depcratetest_nested_mut_to_const {
() => {
// Module: crate
// Provides: {"test_nested_mut_to_const"}
// Dependencies: {}
# [test] fn test_nested_mut_to_const () { let mut s = Bar { foo : Foo { a : 1 , b : 2 } , a : 3 , } ; let mut sr = s . into_partial_ref_mut () ; split_borrow ! (foo_a = & (PartFoo | PartA) sr) ; * sr . part_mut (PartA) = * foo_a . part (PartFoo | PartA) ; * sr . part_mut (PartFoo | PartB) = 10 ; assert_eq ! (s . foo . a , 1) ; assert_eq ! (s . foo . b , 10) ; assert_eq ! (s . a , 1) ; }
};
}
