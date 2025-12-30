// Generated macro for impl_45 (impl)
macro_rules! Depcrate_wasm_bindgen_test_runnerimpl_45 {
() => {
// Module: crate::wasm_bindgen_test_runner
// Provides: {"impl_45"}
// Dependencies: {}
impl TestMode { fn is_worker (self) -> bool { matches ! (self , Self :: DedicatedWorker { .. } | Self :: SharedWorker { .. } | Self :: ServiceWorker { .. }) } fn no_modules (self) -> bool { match self { Self :: Deno => true , Self :: Browser { no_modules } | Self :: Node { no_modules } | Self :: DedicatedWorker { no_modules } | Self :: SharedWorker { no_modules } | Self :: ServiceWorker { no_modules } => no_modules , } } fn env (self) -> & 'static str { match self { TestMode :: Node { .. } => "WASM_BINDGEN_USE_NODE_EXPERIMENTAL" , TestMode :: Deno => "WASM_BINDGEN_USE_DENO" , TestMode :: Browser { .. } => "WASM_BINDGEN_USE_BROWSER" , TestMode :: DedicatedWorker { .. } => "WASM_BINDGEN_USE_DEDICATED_WORKER" , TestMode :: SharedWorker { .. } => "WASM_BINDGEN_USE_SHARED_WORKER" , TestMode :: ServiceWorker { .. } => "WASM_BINDGEN_USE_SERVICE_WORKER" , } } }
};
}
