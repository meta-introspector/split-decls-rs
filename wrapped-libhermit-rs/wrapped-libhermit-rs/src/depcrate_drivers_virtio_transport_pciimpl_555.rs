// Generated macro for impl_555 (impl)
macro_rules! Depcrate_drivers_virtio_transport_pciimpl_555 {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"impl_555"}
// Dependencies: {}
impl Drop for ShMem { fn drop (& mut self) { for i in 0 .. self . len { unsafe { * (self . ptr . add (i)) = 0 ; } } } }
};
}
