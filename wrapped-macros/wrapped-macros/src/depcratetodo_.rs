// Generated macro for todo_ (function)
macro_rules! Depcratetodo_ {
() => {
// Module: crate
// Provides: {"todo_"}
// Dependencies: {}
# [proc_macro] # [proc_macro_error] pub fn todo_ (args : TokenStream) -> TokenStream { function_like :: panic_like :: expand (args , "panicked at 'not yet implemented'" , | format_string | { format ! ("panicked at 'not yet implemented: {format_string}'") }) }
};
}
