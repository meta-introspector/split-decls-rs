// Generated macro for link_to (function)
macro_rules! Depcratelink_to {
() => {
// Module: crate
// Provides: {"link_to"}
// Dependencies: {}
# [doc = " This macro takes a JS module as input and returns a URL that can be used to"] # [doc = " access it at runtime."] # [doc = ""] # [doc = " The module can be specified in a few ways:"] # [doc = " - You can use `inline_js = \"...\"` to create an inline JS file."] # [doc = " - You can use `module = \"/foo/bar\"` to reference a file relative to the"] # [doc = "   root of the crate the macro is invoked in."] # [doc = ""] # [doc = " The returned URL can be used for things like creating workers/worklets:"] # [doc = " ```no_run"] # [doc = " use web_sys::Worker;"] # [doc = " let worker = Worker::new(&wasm_bindgen::link_to!(module = \"/src/worker.js\"));"] # [doc = " ```"] # [proc_macro] pub fn link_to (input : TokenStream) -> TokenStream { match wasm_bindgen_macro_support :: expand_link_to (input . into ()) { Ok (tokens) => { if cfg ! (xxx_debug_only_print_generated_code) { println ! ("{}" , tokens) ; } tokens . into () } Err (diagnostic) => (quote ! { String :: clone (# diagnostic) }) . into () , } }
};
}
