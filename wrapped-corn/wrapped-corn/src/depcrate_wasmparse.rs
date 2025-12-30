// Generated macro for parse (function)
macro_rules! Depcrate_wasmparse {
() => {
// Module: crate::wasm
// Provides: {"parse"}
// Dependencies: {}
# [wasm_bindgen] pub fn parse (corn : & str) -> Result < JsValue , JsValue > { console_error_panic_hook :: set_once () ; let res = crate :: parse (corn) ; match res { Ok (parsed) => Ok (to_value (& parsed) . expect ("Failed to convert struct into js value")) , Err (err) => Err (JsValue :: from_str (err . to_string () . as_str ())) , } }
};
}
