// Generated macro for abort_call_site (macro)
macro_rules! Depcrate_macrosabort_call_site {
() => {
// Module: crate::macros
// Provides: {"abort_call_site"}
// Dependencies: {}
# [doc = " Shortcut for `abort!(Span::call_site(), msg...)`. This macro"] # [doc = " is still preferable over plain panic, panics are not for error reporting."] # [doc = ""] # [doc = " # Syntax"] # [doc = ""] # [doc = " See [the guide](index.html#guide)."] # [doc = ""] # [macro_export] macro_rules ! abort_call_site { ($ ($ tts : tt) *) => { $ crate :: abort ! ($ crate :: __export :: proc_macro2 :: Span :: call_site () , $ ($ tts) *) } ; }
};
}
