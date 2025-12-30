// Generated macro for impl_94 (impl)
macro_rules! Depcrateimpl_94 {
() => {
// Module: crate
// Provides: {"impl_94"}
// Dependencies: {}
impl Iterator { fn looks_like_iterator (it : & JsValue) -> bool { # [wasm_bindgen] extern "C" { type MaybeIterator ; # [wasm_bindgen (method , getter)] fn next (this : & MaybeIterator) -> JsValue ; } if ! it . is_object () { return false ; } let it = it . unchecked_ref :: < MaybeIterator > () ; it . next () . is_function () } }
};
}
