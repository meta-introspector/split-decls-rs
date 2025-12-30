// Generated macro for other_159 (other)
macro_rules! Depcrateother_159 {
() => {
// Module: crate
// Provides: {"other_159"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = Object , typescript_type = "WeakRef<object>")] # [derive (Clone , Debug , PartialEq , Eq)] pub type WeakRef ; # [doc = " The `WeakRef` object contains a weak reference to an object. A weak"] # [doc = " reference to an object is a reference that does not prevent the object"] # [doc = " from being reclaimed by the garbage collector."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakRef)"] # [wasm_bindgen (constructor)] pub fn new (target : & Object) -> WeakRef ; # [doc = " Returns the `Object` this `WeakRef` points to, or `None` if the"] # [doc = " object has been garbage collected."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakRef/deref)"] # [wasm_bindgen (method)] pub fn deref (this : & WeakRef) -> Option < Object > ; }
};
}
