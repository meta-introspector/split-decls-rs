// Generated macro for impl_645 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueueimpl_645 {
() => {
// Module: crate::drivers::virtio::virtqueue
// Provides: {"impl_645"}
// Dependencies: {}
impl AvailBufferToken { # [doc = " Returns the overall number of descriptors."] fn num_descr (& self) -> u16 { u16 :: try_from (self . send_buff . len () + self . recv_buff . len ()) . unwrap () } }
};
}
