// Generated macro for impl_51 (impl)
macro_rules! Depcrate_comimpl_51 {
() => {
// Module: crate::com
// Provides: {"impl_51"}
// Dependencies: {}
impl Clone for ComPtr { fn clone (& self) -> Self { unsafe { com_call ! (IUnknown_Vtbl , self . AddRef ()) ; } Self (self . 0) } }
};
}
