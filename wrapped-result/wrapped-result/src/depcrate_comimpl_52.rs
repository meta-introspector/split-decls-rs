// Generated macro for impl_52 (impl)
macro_rules! Depcrate_comimpl_52 {
() => {
// Module: crate::com
// Provides: {"impl_52"}
// Dependencies: {}
impl Drop for ComPtr { fn drop (& mut self) { unsafe { com_call ! (IUnknown_Vtbl , self . Release ()) ; } } }
};
}
