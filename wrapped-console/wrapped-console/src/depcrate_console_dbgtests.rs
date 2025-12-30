// Generated macro for tests (module)
macro_rules! Depcrate_console_dbgtests {
() => {
// Module: crate::console_dbg
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # ! [allow (dead_code)] # ! [doc = " These exist to ensure code compiles"] use wasm_bindgen :: JsValue ; fn console_works () { console ! () ; { let js_value = JsValue :: from ("test") ; console ! (js_value) ; } { let js_value_1 = JsValue :: from ("test 1") ; let js_value_2 = JsValue :: from ("test 2") ; console ! (js_value_1 , js_value_2) ; } } fn console_dbg_works () { # [derive (Debug)] struct Value (& 'static str) ; console_dbg ! () ; { let value = Value ("test") ; console_dbg ! (value) ; } { let value_1 = Value ("test 1") ; let value_2 = Value ("test 2") ; console_dbg ! (value_1 , value_2) ; } } }
};
}
