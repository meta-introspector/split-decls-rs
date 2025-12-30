// Generated macro for trace (function)
macro_rules! Depcratetrace {
() => {
// Module: crate
// Provides: {"trace"}
// Dependencies: {}
# [proc_macro] # [proc_macro_error] pub fn trace (args : TokenStream) -> TokenStream { function_like :: log :: expand (Level :: Trace , args) }
};
}
