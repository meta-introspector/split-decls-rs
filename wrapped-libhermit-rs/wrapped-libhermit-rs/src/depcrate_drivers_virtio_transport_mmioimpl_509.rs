// Generated macro for impl_509 (impl)
macro_rules! Depcrate_drivers_virtio_transport_mmioimpl_509 {
() => {
// Module: crate::drivers::virtio::transport::mmio
// Provides: {"impl_509"}
// Dependencies: {}
impl IsrStatus { pub fn new (registers : VolatileRef < '_ , DeviceRegisters >) -> Self { let raw = unsafe { mem :: transmute :: < VolatileRef < '_ , _ > , VolatileRef < 'static , _ > > (registers) } ; Self { raw } } pub fn is_queue_interrupt (& self) -> InterruptStatus { self . raw . as_ptr () . interrupt_status () . read () } pub fn acknowledge (& mut self) { let ptr = self . raw . as_mut_ptr () ; let status = ptr . interrupt_status () . read () ; ptr . interrupt_ack () . write (status) ; } }
};
}
