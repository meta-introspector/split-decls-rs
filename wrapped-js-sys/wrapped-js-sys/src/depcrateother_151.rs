// Generated macro for other_151 (other)
macro_rules! Depcrateother_151 {
() => {
// Module: crate
// Provides: {"other_151"}
// Dependencies: {}
# [wasm_bindgen] extern "C" { # [doc = " The `entries()` method returns a new Iterator object that contains an"] # [doc = " array of [value, value] for each element in the Set object, in insertion"] # [doc = " order. For Set objects there is no key like in Map objects. However, to"] # [doc = " keep the API similar to the Map object, each entry has the same value"] # [doc = " for its key and value here, so that an array [value, value] is returned."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/entries)"] # [wasm_bindgen (method)] pub fn entries (set : & Set) -> Iterator ; # [doc = " The `keys()` method is an alias for this method (for similarity with"] # [doc = " Map objects); it behaves exactly the same and returns values"] # [doc = " of Set elements."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/values)"] # [wasm_bindgen (method)] pub fn keys (set : & Set) -> Iterator ; # [doc = " The `values()` method returns a new Iterator object that contains the"] # [doc = " values for each element in the Set object in insertion order."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/values)"] # [wasm_bindgen (method)] pub fn values (set : & Set) -> Iterator ; }
};
}
