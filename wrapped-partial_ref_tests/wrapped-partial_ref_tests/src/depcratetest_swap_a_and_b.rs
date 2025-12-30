// Generated macro for test_swap_a_and_b (function)
macro_rules! Depcratetest_swap_a_and_b {
() => {
// Module: crate
// Provides: {"test_swap_a_and_b"}
// Dependencies: {}
# [test] fn test_swap_a_and_b () { let mut f = Foo { a : 1 , b : 2 } ; let mut fr = f . into_partial_ref_mut () ; swap_a_and_b (fr . borrow ()) ; assert_eq ! (* fr . part (PartA) , 2) ; assert_eq ! (* fr . part (PartB) , 1) ; }
};
}
