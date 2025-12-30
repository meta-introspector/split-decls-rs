// Generated macro for other_72 (other)
macro_rules! Depcrateother_72 {
() => {
// Module: crate
// Provides: {"other_72"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = Object , is_type_of = | v | v . as_bool () . is_some () , typescript_type = "boolean")] # [derive (Clone , PartialEq , Eq)] pub type Boolean ; # [doc = " The `Boolean()` constructor creates an object wrapper for a boolean value."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Boolean)"] # [wasm_bindgen (constructor)] # [deprecated (note = "recommended to use `Boolean::from` instead")] # [allow (deprecated)] pub fn new (value : & JsValue) -> Boolean ; # [doc = " The `valueOf()` method returns the primitive value of a `Boolean` object."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Boolean/valueOf)"] # [wasm_bindgen (method , js_name = valueOf)] pub fn value_of (this : & Boolean) -> bool ; }
};
}
