// Generated macro for impl_321 (impl)
macro_rules! Depcrate_client_legacy_connectimpl_321 {
() => {
// Module: crate::client::legacy::connect
// Provides: {"impl_321"}
// Dependencies: {}
impl < T > ExtraInner for ExtraChain < T > where T : Clone + Send + Sync + 'static , { fn clone_box (& self) -> Box < dyn ExtraInner > { Box :: new (self . clone ()) } fn set (& self , res : & mut Extensions) { self . 0 . set (res) ; res . insert (self . 1 . clone ()) ; } }
};
}
