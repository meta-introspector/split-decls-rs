// Generated macro for impl_549 (impl)
macro_rules! Depcrate_drivers_virtio_transport_pciimpl_549 {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"impl_549"}
// Dependencies: {}
impl IsrStatus { fn new (raw : VolatileRef < 'static , IsrStatusRaw >) -> Self { IsrStatus { isr_stat : raw } } pub fn is_queue_interrupt (& self) -> IsrStatusRaw { self . isr_stat . as_ptr () . read () } pub fn acknowledge (& mut self) { } }
};
}
