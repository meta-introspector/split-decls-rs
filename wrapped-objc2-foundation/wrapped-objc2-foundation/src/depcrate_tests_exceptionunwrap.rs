// Generated macro for unwrap (function)
macro_rules! Depcrate_tests_exceptionunwrap {
() => {
// Module: crate::tests::exception
// Provides: {"unwrap"}
// Dependencies: {}
# [test] # [should_panic = "'abc' reason: def"] fn unwrap () { let exc = NSException :: new (ns_string ! ("abc") , Some (ns_string ! ("def")) , None) . unwrap () ; panic ! ("{exc:?}") ; }
};
}
