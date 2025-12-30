// Generated macro for arbitrary_take_rest_for_bytes (function)
macro_rules! Depcrate_testsarbitrary_take_rest_for_bytes {
() => {
// Module: crate::tests
// Provides: {"arbitrary_take_rest_for_bytes"}
// Dependencies: {}
# [test] fn arbitrary_take_rest_for_bytes () { let x = [1 , 2 , 3 , 4] ; let buf = Unstructured :: new (& x) ; let expected = & [1 , 2 , 3 , 4] ; let actual = checked_arbitrary_take_rest :: < & [u8] > (buf) . unwrap () ; assert_eq ! (expected , actual) ; }
};
}
