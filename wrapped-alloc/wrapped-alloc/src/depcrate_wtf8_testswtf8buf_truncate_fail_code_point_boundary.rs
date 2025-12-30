// Generated macro for wtf8buf_truncate_fail_code_point_boundary (function)
macro_rules! Depcrate_wtf8_testswtf8buf_truncate_fail_code_point_boundary {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8buf_truncate_fail_code_point_boundary"}
// Dependencies: {}
# [test] # [should_panic] fn wtf8buf_truncate_fail_code_point_boundary () { let mut string = Wtf8Buf :: from_str ("aé") ; string . truncate (2) ; }
};
}
