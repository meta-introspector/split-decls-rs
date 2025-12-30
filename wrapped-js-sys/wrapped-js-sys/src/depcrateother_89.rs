// Generated macro for other_89 (other)
macro_rules! Depcrateother_89 {
() => {
// Module: crate
// Provides: {"other_89"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = Object , typescript_type = "Generator<any, any, any>")] # [derive (Clone , Debug , PartialEq , Eq)] pub type Generator ; # [doc = " The `next()` method returns an object with two properties done and value."] # [doc = " You can also provide a parameter to the next method to send a value to the generator."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator/next)"] # [wasm_bindgen (method , structural , catch)] pub fn next (this : & Generator , value : & JsValue) -> Result < JsValue , JsValue > ; # [doc = " The `return()` method returns the given value and finishes the generator."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator/return)"] # [wasm_bindgen (method , structural , js_name = return)] pub fn return_ (this : & Generator , value : & JsValue) -> JsValue ; # [doc = " The `throw()` method resumes the execution of a generator by throwing an error into it"] # [doc = " and returns an object with two properties done and value."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator/throw)"] # [wasm_bindgen (method , structural , catch)] pub fn throw (this : & Generator , error : & Error) -> Result < JsValue , JsValue > ; }
};
}
