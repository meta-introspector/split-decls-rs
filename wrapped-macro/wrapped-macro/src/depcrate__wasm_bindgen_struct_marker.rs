// Generated macro for __wasm_bindgen_struct_marker (function)
macro_rules! Depcrate__wasm_bindgen_struct_marker {
() => {
// Module: crate
// Provides: {"__wasm_bindgen_struct_marker"}
// Dependencies: {}
# [proc_macro_derive (BindgenedStruct , attributes (wasm_bindgen))] pub fn __wasm_bindgen_struct_marker (item : TokenStream) -> TokenStream { match wasm_bindgen_macro_support :: expand_struct_marker (item . into ()) { Ok (tokens) => { if cfg ! (xxx_debug_only_print_generated_code) { println ! ("{}" , tokens) ; } tokens . into () } Err (diagnostic) => (quote ! { # diagnostic }) . into () , } }
};
}
