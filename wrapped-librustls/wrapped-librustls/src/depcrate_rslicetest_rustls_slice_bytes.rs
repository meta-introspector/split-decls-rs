// Generated macro for test_rustls_slice_bytes (function)
macro_rules! Depcrate_rslicetest_rustls_slice_bytes {
() => {
// Module: crate::rslice
// Provides: {"test_rustls_slice_bytes"}
// Dependencies: {}
# [test] fn test_rustls_slice_bytes () { let bytes = b"abcd" ; let rsb : rustls_slice_bytes = bytes . as_ref () . into () ; unsafe { assert_eq ! (* rsb . data , b'a') ; assert_eq ! (* rsb . data . offset (3) , b'd') ; assert_eq ! (rsb . len , 4) ; } }
};
}
