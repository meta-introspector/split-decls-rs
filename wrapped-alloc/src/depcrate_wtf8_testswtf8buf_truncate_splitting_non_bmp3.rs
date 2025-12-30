// Generated macro for wtf8buf_truncate_splitting_non_bmp3 (function)
macro_rules! Depcrate_wtf8_testswtf8buf_truncate_splitting_non_bmp3 {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8buf_truncate_splitting_non_bmp3"}
// Dependencies: {}
# [test] # [should_panic] fn wtf8buf_truncate_splitting_non_bmp3 () { let mut string = Wtf8Buf :: from_str ("💩") ; assert ! (string . is_known_utf8) ; string . truncate (3) ; }
};
}
