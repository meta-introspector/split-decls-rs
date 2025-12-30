// Generated macro for impl_432 (impl)
macro_rules! Depcrate_drivers_net_virtioimpl_432 {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"impl_432"}
// Dependencies: {}
impl Driver for VirtioNetDriver < Init > { fn get_interrupt_number (& self) -> InterruptLine { self . irq } fn get_name (& self) -> & 'static str { "virtio" } }
};
}
