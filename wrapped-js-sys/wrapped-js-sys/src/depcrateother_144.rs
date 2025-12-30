// Generated macro for other_144 (other)
macro_rules! Depcrateother_144 {
() => {
// Module: crate
// Provides: {"other_144"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (typescript_type = "ProxyConstructor")] # [derive (Clone , Debug)] pub type Proxy ; # [doc = " The [`Proxy`] object is used to define custom behavior for fundamental"] # [doc = " operations (e.g. property lookup, assignment, enumeration, function"] # [doc = " invocation, etc)."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy)"] # [wasm_bindgen (constructor)] pub fn new (target : & JsValue , handler : & Object) -> Proxy ; # [doc = " The `Proxy.revocable()` method is used to create a revocable [`Proxy`]"] # [doc = " object."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy/revocable)"] # [wasm_bindgen (static_method_of = Proxy)] pub fn revocable (target : & JsValue , handler : & Object) -> Object ; }
};
}
