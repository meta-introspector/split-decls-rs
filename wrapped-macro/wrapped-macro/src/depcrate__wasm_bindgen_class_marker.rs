// Generated macro for __wasm_bindgen_class_marker (function)
macro_rules! Depcrate__wasm_bindgen_class_marker {
() => {
// Module: crate
// Provides: {"__wasm_bindgen_class_marker"}
// Dependencies: {}
# [proc_macro_attribute] pub fn __wasm_bindgen_class_marker (attr : TokenStream , input : TokenStream) -> TokenStream { match wasm_bindgen_macro_support :: expand_class_marker (attr . into () , input . into ()) { Ok (tokens) => { if cfg ! (xxx_debug_only_print_generated_code) { println ! ("{}" , tokens) ; } tokens . into () } Err (diagnostic) => (quote ! { # diagnostic }) . into () , } }
};
}
