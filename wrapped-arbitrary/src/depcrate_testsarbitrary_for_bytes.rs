// Generated macro for arbitrary_for_bytes (function)
macro_rules! Depcrate_testsarbitrary_for_bytes {
() => {
// Module: crate::tests
// Provides: {"arbitrary_for_bytes"}
// Dependencies: {}
# [test] fn arbitrary_for_bytes () { let x = [1 , 2 , 3 , 4 , 4] ; let mut buf = Unstructured :: new (& x) ; let expected = & [1 , 2 , 3 , 4] ; let actual = checked_arbitrary :: < & [u8] > (& mut buf) . unwrap () ; assert_eq ! (expected , actual) ; }
};
}
