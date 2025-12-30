// Generated macro for other_93 (other)
macro_rules! Depcrateother_93 {
() => {
// Module: crate
// Provides: {"other_93"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [doc = " Any object that conforms to the JS iterator protocol. For example,"] # [doc = " something returned by `myArray[Symbol.iterator]()`."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols)"] # [derive (Clone , Debug)] # [wasm_bindgen (is_type_of = Iterator :: looks_like_iterator , typescript_type = "Iterator<any>")] pub type Iterator ; # [doc = " The `next()` method always has to return an object with appropriate"] # [doc = " properties including done and value. If a non-object value gets returned"] # [doc = " (such as false or undefined), a TypeError (\"iterator.next() returned a"] # [doc = " non-object value\") will be thrown."] # [wasm_bindgen (catch , method , structural)] pub fn next (this : & Iterator) -> Result < IteratorNext , JsValue > ; }
};
}
