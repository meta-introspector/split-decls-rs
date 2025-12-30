// Generated macro for test_eq_operator (function)
macro_rules! Depcrate_teststest_eq_operator {
() => {
// Module: crate::tests
// Provides: {"test_eq_operator"}
// Dependencies: {}
# [allow (clippy :: cmp_owned)] # [allow (clippy :: op_ref)] # [test] fn test_eq_operator () { let x = CompactString :: const_new ("foo") ; let y = x . clone () ; macro_rules ! test_impl { ($ a : expr , $ b : expr) => { let _ = $ a == $ b ; let _ = &$ a == $ b ; let _ = &$ a == &$ b ; let _ = $ b == $ a ; let _ = &$ b == $ a ; let _ = &$ b == &$ a ; } ; } test_impl ! ("a" , x) ; test_impl ! (String :: from ("a") , x) ; test_impl ! (Cow :: Borrowed ("a") , x) ; test_impl ! (y , x) ; }
};
}
