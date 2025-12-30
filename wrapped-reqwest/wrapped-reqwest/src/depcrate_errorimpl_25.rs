// Generated macro for impl_25 (impl)
macro_rules! Depcrate_errorimpl_25 {
() => {
// Module: crate::error
// Provides: {"impl_25"}
// Dependencies: {}
# [cfg (target_arch = "wasm32")] impl From < crate :: error :: Error > for wasm_bindgen :: JsValue { fn from (err : Error) -> wasm_bindgen :: JsValue { js_sys :: Error :: from (err) . into () } }
};
}
