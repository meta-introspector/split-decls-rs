// Generated macro for pending (function)
macro_rules! Depcrate_ssl_testpending {
() => {
// Module: crate::ssl::test
// Provides: {"pending"}
// Dependencies: {}
# [test] fn pending () { let mut server = Server :: builder () ; server . io_cb (| mut s | s . write_all (& [0 ; 10]) . unwrap ()) ; let server = server . build () ; let mut s = server . client () . connect () ; s . read_exact (& mut [0]) . unwrap () ; assert_eq ! (s . ssl () . pending () , 9) ; assert_eq ! (s . read (& mut [0 ; 10]) . unwrap () , 9) ; }
};
}
