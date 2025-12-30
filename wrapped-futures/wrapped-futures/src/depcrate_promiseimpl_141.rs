// Generated macro for impl_141 (impl)
macro_rules! Depcrate_promiseimpl_141 {
() => {
// Module: crate::promise
// Provides: {"impl_141"}
// Dependencies: {}
impl < T > Drop for Promise < T > where T : Send + 'static , { fn drop (& mut self) { if let Some (cancel_token) = self . cancel_token . take () { self . inner . slot . cancel (cancel_token) } } }
};
}
