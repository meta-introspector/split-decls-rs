// Generated macro for impl_430 (impl)
macro_rules! Depcrate_sync_mpscimpl_430 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_430"}
// Dependencies: {}
impl < T > Clone for Sender < T > { fn clone (& self) -> Sender < T > { Sender { object : std :: sync :: Arc :: clone (& self . object) , sender : self . sender . clone () , } } }
};
}
