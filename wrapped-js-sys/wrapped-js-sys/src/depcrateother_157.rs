// Generated macro for other_157 (other)
macro_rules! Depcrateother_157 {
() => {
// Module: crate
// Provides: {"other_157"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [wasm_bindgen (extends = Object , typescript_type = "WeakSet<object>")] # [derive (Clone , Debug , PartialEq , Eq)] pub type WeakSet ; # [doc = " The `WeakSet` object lets you store weakly held objects in a collection."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet)"] # [wasm_bindgen (constructor)] pub fn new () -> WeakSet ; # [doc = " The `has()` method returns a boolean indicating whether an object exists"] # [doc = " in a WeakSet or not."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/has)"] # [wasm_bindgen (method)] pub fn has (this : & WeakSet , value : & Object) -> bool ; # [doc = " The `add()` method appends a new object to the end of a WeakSet object."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/add)"] # [wasm_bindgen (method)] pub fn add (this : & WeakSet , value : & Object) -> WeakSet ; # [doc = " The `delete()` method removes the specified element from a WeakSet"] # [doc = " object."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/delete)"] # [wasm_bindgen (method)] pub fn delete (this : & WeakSet , value : & Object) -> bool ; }
};
}
