// Generated macro for other_2 (other)
macro_rules! Depcrateother_2 {
() => {
// Module: crate
// Provides: {"other_2"}
// Dependencies: {}
# [wasm_bindgen (module = "/defined-in-js.js")] extern "C" { type MyClass ; # [wasm_bindgen (constructor)] fn new () -> MyClass ; # [wasm_bindgen (method , getter)] fn number (this : & MyClass) -> u32 ; # [wasm_bindgen (method , setter)] fn set_number (this : & MyClass , number : u32) -> MyClass ; # [wasm_bindgen (method)] fn render (this : & MyClass) -> String ; }
};
}
