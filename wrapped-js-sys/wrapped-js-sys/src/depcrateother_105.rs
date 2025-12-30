// Generated macro for other_105 (other)
macro_rules! Depcrateother_105 {
() => {
// Module: crate
// Provides: {"other_105"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [doc = " The result of calling `next()` on a JS iterator."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols)"] # [wasm_bindgen (extends = Object , typescript_type = "IteratorResult<any>")] # [derive (Clone , Debug , PartialEq , Eq)] pub type IteratorNext ; # [doc = " Has the value `true` if the iterator is past the end of the iterated"] # [doc = " sequence. In this case value optionally specifies the return value of"] # [doc = " the iterator."] # [doc = ""] # [doc = " Has the value `false` if the iterator was able to produce the next value"] # [doc = " in the sequence. This is equivalent of not specifying the done property"] # [doc = " altogether."] # [wasm_bindgen (method , getter , structural)] pub fn done (this : & IteratorNext) -> bool ; # [doc = " Any JavaScript value returned by the iterator. Can be omitted when done"] # [doc = " is true."] # [wasm_bindgen (method , getter , structural)] pub fn value (this : & IteratorNext) -> JsValue ; }
};
}
