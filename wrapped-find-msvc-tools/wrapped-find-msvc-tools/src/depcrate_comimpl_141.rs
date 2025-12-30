// Generated macro for impl_141 (impl)
macro_rules! Depcrate_comimpl_141 {
() => {
// Module: crate::com
// Provides: {"impl_141"}
// Dependencies: {}
impl < T > Deref for ComPtr < T > where T : Interface , { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . 0 } } }
};
}
