// Generated macro for warn (function)
macro_rules! Depcratewarn {
() => {
// Module: crate
// Provides: {"warn"}
// Dependencies: {}
# [proc_macro] # [proc_macro_error] pub fn warn (args : TokenStream) -> TokenStream { function_like :: log :: expand (Level :: Warn , args) }
};
}
