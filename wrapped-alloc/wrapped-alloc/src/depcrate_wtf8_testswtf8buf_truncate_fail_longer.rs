// Generated macro for wtf8buf_truncate_fail_longer (function)
macro_rules! Depcrate_wtf8_testswtf8buf_truncate_fail_longer {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8buf_truncate_fail_longer"}
// Dependencies: {}
# [test] # [should_panic] fn wtf8buf_truncate_fail_longer () { let mut string = Wtf8Buf :: from_str ("aé") ; string . truncate (4) ; }
};
}
