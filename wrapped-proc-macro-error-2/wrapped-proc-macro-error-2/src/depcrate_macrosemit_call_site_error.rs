// Generated macro for emit_call_site_error (macro)
macro_rules! Depcrate_macrosemit_call_site_error {
() => {
// Module: crate::macros
// Provides: {"emit_call_site_error"}
// Dependencies: {}
# [doc = " Shortcut for `emit_error!(Span::call_site(), ...)`. This macro"] # [doc = " is still preferable over plain panic, panics are not for error reporting.."] # [doc = ""] # [doc = " # Syntax"] # [doc = ""] # [doc = " See [the guide](index.html#guide)."] # [doc = ""] # [macro_export] macro_rules ! emit_call_site_error { ($ ($ tts : tt) *) => { $ crate :: emit_error ! ($ crate :: __export :: proc_macro2 :: Span :: call_site () , $ ($ tts) *) } ; }
};
}
