// Generated macro for impl_318 (impl)
macro_rules! Depcrate_client_legacy_connectimpl_318 {
() => {
// Module: crate::client::legacy::connect
// Provides: {"impl_318"}
// Dependencies: {}
impl < T > ExtraInner for ExtraEnvelope < T > where T : Clone + Send + Sync + 'static , { fn clone_box (& self) -> Box < dyn ExtraInner > { Box :: new (self . clone ()) } fn set (& self , res : & mut Extensions) { res . insert (self . 0 . clone ()) ; } }
};
}
