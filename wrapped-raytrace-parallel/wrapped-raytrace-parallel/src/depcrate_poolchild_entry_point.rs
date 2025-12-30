// Generated macro for child_entry_point (function)
macro_rules! Depcrate_poolchild_entry_point {
() => {
// Module: crate::pool
// Provides: {"child_entry_point"}
// Dependencies: {}
# [doc = " Entry point invoked by `worker.js`, a bit of a hack but see the \"TODO\" above"] # [doc = " about `worker.js` in general."] # [wasm_bindgen] pub fn child_entry_point (ptr : u32) -> Result < () , JsValue > { let ptr = unsafe { Box :: from_raw (ptr as * mut Work) } ; let global = js_sys :: global () . unchecked_into :: < DedicatedWorkerGlobalScope > () ; (ptr . func) () ; global . post_message (& JsValue :: undefined ()) ? ; Ok (()) }
};
}
