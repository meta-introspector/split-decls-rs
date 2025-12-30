// Generated macro for test_eq_ord (function)
macro_rules! Depcrate_implstest_eq_ord {
() => {
// Module: crate::impls
// Provides: {"test_eq_ord"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn test_eq_ord () { use core :: cmp :: Ordering ; use crate :: { BStr , BString } ; let b = BStr :: new ("hello") ; assert_eq ! (b , b"hello") ; assert_ne ! (b , b"world") ; assert_eq ! (b . partial_cmp (b"hello") , Some (Ordering :: Equal)) ; assert_eq ! (b . partial_cmp (b"world") , Some (Ordering :: Less)) ; let b = BString :: from ("hello") ; assert_eq ! (b , b"hello") ; assert_ne ! (b , b"world") ; assert_eq ! (b . partial_cmp (b"hello") , Some (Ordering :: Equal)) ; assert_eq ! (b . partial_cmp (b"world") , Some (Ordering :: Less)) ; }
};
}
