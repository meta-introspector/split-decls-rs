// Generated macro for unwrap_panic (function)
macro_rules! Depcrate_testunwrap_panic {
() => {
// Module: crate::test
// Provides: {"unwrap_panic"}
// Dependencies: {}
# [test] # [should_panic] fn unwrap_panic () { let _ = convert (vec ! [Ok (0) , Err (())] . into_iter ()) . unwrap () . collect :: < Vec < _ > > () ; }
};
}
