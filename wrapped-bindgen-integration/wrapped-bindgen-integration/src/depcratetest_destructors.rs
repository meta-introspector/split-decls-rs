// Generated macro for test_destructors (function)
macro_rules! Depcratetest_destructors {
() => {
// Module: crate
// Provides: {"test_destructors"}
// Dependencies: {}
# [test] fn test_destructors () { let mut v = true ; { let auto_restore = unsafe { bindings :: AutoRestoreBool :: new (& mut v) } ; v = false ; } assert ! (v , "Should've been restored when going out of scope") ; }
};
}
