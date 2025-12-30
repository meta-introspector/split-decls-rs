// Generated macro for impl_314 (impl)
macro_rules! Depcrate_wit_nonstandardimpl_314 {
() => {
// Module: crate::wit::nonstandard
// Provides: {"impl_314"}
// Dependencies: {}
impl AuxReceiverKind { # [doc = " Returns whether this is `AuxReceiverKind::None` (in other words,"] # [doc = " whether the method with this receiver is static)."] pub fn is_static (self) -> bool { matches ! (self , Self :: None) } }
};
}
