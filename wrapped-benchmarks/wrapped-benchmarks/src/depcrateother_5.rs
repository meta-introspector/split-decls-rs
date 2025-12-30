// Generated macro for other_5 (other)
macro_rules! Depcrateother_5 {
() => {
// Module: crate
// Provides: {"other_5"}
// Dependencies: {}
# [wasm_bindgen (raw_module = "../globals.js")] extern "C" { # [wasm_bindgen (js_name = jsthunk)] fn js_thunk () ; # [wasm_bindgen (js_name = add)] fn js_add (a : i32 , b : i32) -> i32 ; # [wasm_bindgen (js_name = use_baz)] fn js_use_baz (val : Baz) ; pub type Foo ; # [wasm_bindgen (method , final , js_name = bar)] fn bar_final (this : & Foo) ; # [wasm_bindgen (method , structural , js_name = bar)] fn bar_structural (this : & Foo) ; # [wasm_bindgen (js_name = jsthunk)] fn doesnt_throw () ; # [wasm_bindgen (catch , js_name = jsthunk)] fn doesnt_throw_catch () -> Result < () , JsValue > ; }
};
}
