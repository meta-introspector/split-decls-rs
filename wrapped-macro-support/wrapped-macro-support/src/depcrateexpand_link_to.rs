// Generated macro for expand_link_to (function)
macro_rules! Depcrateexpand_link_to {
() => {
// Module: crate
// Provides: {"expand_link_to"}
// Dependencies: {}
# [doc = " Takes the parsed input from a `wasm_bindgen::link_to` macro and returns the generated link"] pub fn expand_link_to (input : TokenStream) -> Result < TokenStream , Diagnostic > { parser :: reset_attrs_used () ; let opts = syn :: parse2 (input) ? ; let mut tokens = proc_macro2 :: TokenStream :: new () ; let link = parser :: link_to (opts) ? ; link . try_to_tokens (& mut tokens) ? ; Ok (tokens) }
};
}
