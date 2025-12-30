// Generated macro for wasm_bindgen (function)
macro_rules! Depcratewasm_bindgen {
() => {
// Module: crate
// Provides: {"wasm_bindgen"}
// Dependencies: {}
# [doc = " A list of all the attributes can be found here: https://wasm-bindgen.github.io/wasm-bindgen/reference/attributes/index.html"] # [proc_macro_attribute] pub fn wasm_bindgen (attr : TokenStream , input : TokenStream) -> TokenStream { match wasm_bindgen_macro_support :: expand (attr . into () , input . into ()) { Ok (tokens) => { if cfg ! (xxx_debug_only_print_generated_code) { println ! ("{}" , tokens) ; } tokens . into () } Err (diagnostic) => (quote ! { # diagnostic }) . into () , } }
};
}
