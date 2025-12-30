// Generated macro for test_nested_const (function)
macro_rules! Depcratetest_nested_const {
() => {
// Module: crate
// Provides: {"test_nested_const"}
// Dependencies: {}
# [test] fn test_nested_const () { let mut s = Bar { foo : Foo { a : 1 , b : 2 } , a : 3 , } ; let mut sr = s . into_partial_ref_mut () ; split_borrow ! (src = & (PartFoo , mut PartA) sr) ; split_borrow ! (foo_a = & (PartFoo | PartA) src) ; * src . part_mut (PartA) = * foo_a . part (PartFoo | PartA) ; assert_eq ! (s . foo . a , 1) ; assert_eq ! (s . a , 1) ; }
};
}
