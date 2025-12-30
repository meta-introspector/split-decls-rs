// Generated macro for impl_proc_macro_error (function)
macro_rules! Depcrateimpl_proc_macro_error {
() => {
// Module: crate
// Provides: {"impl_proc_macro_error"}
// Dependencies: {}
fn impl_proc_macro_error (attr : TokenStream2 , input : TokenStream2) -> Result < TokenStream > { let (attrs , signature , body) = parse_input (input) ? ; let mut settings = parse_settings (attr) ? ; let is_proc_macro = is_proc_macro (& attrs) ; if is_proc_macro { settings . set (AssertUnwindSafe) ; } if detect_proc_macro_hack (& attrs) { settings . set (ProcMacroHack) ; } if settings . is_set (ProcMacroHack) { settings . set (AllowNotMacro) ; } if ! (settings . is_set (AllowNotMacro) || is_proc_macro) { return Err (Error :: new (Span :: call_site () , "#[proc_macro_error] attribute can be used only with procedural macros\n\n  \
            = hint: if you are really sure that #[proc_macro_error] should be applied \
            to this exact function, use #[proc_macro_error(allow_not_macro)]\n" . into () ,)) ; } let body = gen_body (& body , & settings) ; let res = quote ! { # (# attrs) * # (# signature) * { # body } } ; Ok (res . into ()) }
};
}
