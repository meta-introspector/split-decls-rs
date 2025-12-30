// Generated macro for generate (function)
macro_rules! Depcrategenerate {
() => {
// Module: crate
// Provides: {"generate"}
// Dependencies: {}
# [proc_macro] pub fn generate (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { syn :: parse_macro_input ! (input as Config) . expand () . unwrap_or_else (Error :: into_compile_error) . into () }
};
}
