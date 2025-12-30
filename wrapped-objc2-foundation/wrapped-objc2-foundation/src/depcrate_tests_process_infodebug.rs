// Generated macro for debug (function)
macro_rules! Depcrate_tests_process_infodebug {
() => {
// Module: crate::tests::process_info
// Provides: {"debug"}
// Dependencies: {}
# [test] fn debug () { let info = NSProcessInfo :: processInfo () ; # [cfg (feature = "NSString")] let expected = format ! ("NSProcessInfo {{ processName: {:?}, .. }}" , info . processName ()) ; # [cfg (not (feature = "NSString"))] let expected = "NSProcessInfo { .. }" ; assert_eq ! (format ! ("{info:?}") , expected) ; }
};
}
