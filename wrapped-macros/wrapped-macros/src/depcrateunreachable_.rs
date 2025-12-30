// Generated macro for unreachable_ (function)
macro_rules! Depcrateunreachable_ {
() => {
// Module: crate
// Provides: {"unreachable_"}
// Dependencies: {}
# [proc_macro] # [proc_macro_error] pub fn unreachable_ (args : TokenStream) -> TokenStream { function_like :: panic_like :: expand (args , "panicked at 'internal error: entered unreachable code'" , | format_string | { format ! ("panicked at 'internal error: entered unreachable code: {}'" , format_string) } ,) }
};
}
