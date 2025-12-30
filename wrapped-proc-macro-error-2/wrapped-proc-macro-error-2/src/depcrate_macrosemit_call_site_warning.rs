// Generated macro for emit_call_site_warning (macro)
macro_rules! Depcrate_macrosemit_call_site_warning {
() => {
// Module: crate::macros
// Provides: {"emit_call_site_warning"}
// Dependencies: {}
# [doc = " Shortcut for `emit_warning!(Span::call_site(), ...)`."] # [doc = ""] # [doc = " **Does nothing on stable**"] # [doc = ""] # [doc = " # Syntax"] # [doc = ""] # [doc = " See [the guide](index.html#guide)."] # [doc = ""] # [macro_export] macro_rules ! emit_call_site_warning { ($ ($ tts : tt) *) => { { $ crate :: emit_warning ! ($ crate :: __export :: proc_macro2 :: Span :: call_site () , $ ($ tts) *) } } ; }
};
}
