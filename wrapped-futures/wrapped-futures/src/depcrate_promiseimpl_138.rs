// Generated macro for impl_138 (impl)
macro_rules! Depcrate_promiseimpl_138 {
() => {
// Module: crate::promise
// Provides: {"impl_138"}
// Dependencies: {}
impl < T > Drop for Complete < T > where T : Send + 'static , { fn drop (& mut self) { if ! self . completed { self . send (None) ; } } }
};
}
