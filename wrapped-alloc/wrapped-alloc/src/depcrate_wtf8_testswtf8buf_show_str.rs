// Generated macro for wtf8buf_show_str (function)
macro_rules! Depcrate_wtf8_testswtf8buf_show_str {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8buf_show_str"}
// Dependencies: {}
# [test] fn wtf8buf_show_str () { let text = "a\té 💩\r" ; let string = Wtf8Buf :: from_str (text) ; assert_eq ! (format ! ("{text:?}") , format ! ("{string:?}")) ; }
};
}
