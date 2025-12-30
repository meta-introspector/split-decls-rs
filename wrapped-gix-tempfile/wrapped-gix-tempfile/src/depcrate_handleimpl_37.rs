// Generated macro for impl_37 (impl)
macro_rules! Depcrate_handleimpl_37 {
() => {
// Module: crate::handle
// Provides: {"impl_37"}
// Dependencies: {}
impl < T : std :: fmt :: Debug > Drop for Handle < T > { fn drop (& mut self) { if let Some ((_id , Some (tempfile))) = REGISTRY . remove (& self . id) { tempfile . drop_impl () ; } } }
};
}
