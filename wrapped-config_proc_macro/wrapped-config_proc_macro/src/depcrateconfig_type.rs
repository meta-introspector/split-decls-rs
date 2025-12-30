// Generated macro for config_type (function)
macro_rules! Depcrateconfig_type {
() => {
// Module: crate
// Provides: {"config_type"}
// Dependencies: {}
# [proc_macro_attribute] pub fn config_type (_args : TokenStream , input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as syn :: Item) ; let output = config_type :: define_config_type (& input) ; # [cfg (feature = "debug-with-rustfmt")] { utils :: debug_with_rustfmt (& output) ; } TokenStream :: from (output) }
};
}
