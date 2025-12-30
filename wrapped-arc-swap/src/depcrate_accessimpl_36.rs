// Generated macro for impl_36 (impl)
macro_rules! Depcrate_accessimpl_36 {
() => {
// Module: crate::access
// Provides: {"impl_36"}
// Dependencies: {}
impl < T : Clone > Access < T > for Constant < T > { type Guard = ConstantDeref < T > ; fn load (& self) -> Self :: Guard { ConstantDeref (self . 0 . clone ()) } }
};
}
