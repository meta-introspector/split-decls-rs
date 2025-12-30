// Generated macro for test_rustls_str (function)
macro_rules! Depcrate_rslicetest_rustls_str {
() => {
// Module: crate::rslice
// Provides: {"test_rustls_str"}
// Dependencies: {}
# [test] fn test_rustls_str () { let s = "abcd" ; let rs : rustls_str = s . try_into () . unwrap () ; assert_eq ! (rs . len , 4) ; unsafe { assert_eq ! (* rs . data , 'a' as c_char) ; assert_eq ! (* rs . data . offset (3) , 'd' as c_char) ; } let rs = unsafe { rs . to_str () } ; assert_eq ! (rs , s) ; }
};
}
