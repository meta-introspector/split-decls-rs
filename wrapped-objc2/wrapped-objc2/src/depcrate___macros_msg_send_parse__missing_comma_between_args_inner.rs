// Generated macro for __missing_comma_between_args_inner (macro)
macro_rules! Depcrate___macros_msg_send_parse__missing_comma_between_args_inner {
() => {
// Module: crate::__macros::msg_send::parse
// Provides: {"__missing_comma_between_args_inner"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __missing_comma_between_args_inner { ($ ($ args : tt) *) => { { # [deprecated = $ crate :: __macros :: concat ! ("using msg_send! without a comma between arguments is " , "technically not valid macro syntax, and may break in a future " , "version of Rust. You should use the following instead:\n" , "msg_send![" , $ ($ args) * "]")] # [inline] fn __missing_comma () { } __missing_comma () ; } } ; }
};
}
