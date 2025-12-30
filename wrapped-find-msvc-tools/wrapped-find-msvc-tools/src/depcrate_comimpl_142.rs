// Generated macro for impl_142 (impl)
macro_rules! Depcrate_comimpl_142 {
() => {
// Module: crate::com
// Provides: {"impl_142"}
// Dependencies: {}
impl < T > Clone for ComPtr < T > where T : Interface , { fn clone (& self) -> Self { unsafe { self . as_unknown () . AddRef () ; ComPtr :: from_raw (self . 0) } } }
};
}
