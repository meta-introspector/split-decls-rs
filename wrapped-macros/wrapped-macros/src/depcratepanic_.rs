// Generated macro for panic_ (function)
macro_rules! Depcratepanic_ {
() => {
// Module: crate
// Provides: {"panic_"}
// Dependencies: {}
# [proc_macro] # [proc_macro_error] pub fn panic_ (args : TokenStream) -> TokenStream { function_like :: panic_like :: expand (args , "panicked at 'explicit panic'" , | format_string | { format ! ("panicked at '{format_string}'") }) }
};
}
