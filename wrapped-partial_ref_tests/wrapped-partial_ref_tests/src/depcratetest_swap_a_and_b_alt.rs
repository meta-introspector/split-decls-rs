// Generated macro for test_swap_a_and_b_alt (function)
macro_rules! Depcratetest_swap_a_and_b_alt {
() => {
// Module: crate
// Provides: {"test_swap_a_and_b_alt"}
// Dependencies: {}
# [test] fn test_swap_a_and_b_alt () { let mut f = Foo { a : 1 , b : 2 } ; let mut fr = f . into_partial_ref_mut () ; split_borrow ! (x = & (mut PartC) fr) ; swap_a_and_b_alt (fr . borrow ()) ; assert_eq ! (* fr . part (PartA) , 2) ; assert_eq ! (* fr . part (PartB) , 1) ; drop (x) ; }
};
}
