// Generated macro for zero_length_buffers (function)
macro_rules! Depcrate_ssl_testzero_length_buffers {
() => {
// Module: crate::ssl::test
// Provides: {"zero_length_buffers"}
// Dependencies: {}
# [test] fn zero_length_buffers () { let server = Server :: builder () . build () ; let mut s = server . client () . connect () ; assert_eq ! (s . write (& []) . unwrap () , 0) ; assert_eq ! (s . read (& mut []) . unwrap () , 0) ; }
};
}
