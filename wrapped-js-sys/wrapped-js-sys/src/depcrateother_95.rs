// Generated macro for other_95 (other)
macro_rules! Depcrateother_95 {
() => {
// Module: crate
// Provides: {"other_95"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [doc = " Any object that conforms to the JS async iterator protocol. For example,"] # [doc = " something returned by `myObject[Symbol.asyncIterator]()`."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for-await...of)"] # [derive (Clone , Debug)] # [wasm_bindgen (is_type_of = Iterator :: looks_like_iterator , typescript_type = "AsyncIterator<any>")] pub type AsyncIterator ; # [doc = " The `next()` method always has to return a Promise which resolves to an object"] # [doc = " with appropriate properties including done and value. If a non-object value"] # [doc = " gets returned (such as false or undefined), a TypeError (\"iterator.next()"] # [doc = " returned a non-object value\") will be thrown."] # [wasm_bindgen (catch , method , structural)] pub fn next (this : & AsyncIterator) -> Result < Promise , JsValue > ; }
};
}
