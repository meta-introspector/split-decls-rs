// Generated macro for impl_143 (impl)
macro_rules! Depcrate_comimpl_143 {
() => {
// Module: crate::com
// Provides: {"impl_143"}
// Dependencies: {}
impl < T > Drop for ComPtr < T > where T : Interface , { fn drop (& mut self) { unsafe { self . as_unknown () . Release () ; } } }
};
}
