// Generated macro for impl_118 (impl)
macro_rules! Depcrate_drivers_consoleimpl_118 {
() => {
// Module: crate::drivers::console
// Provides: {"impl_118"}
// Dependencies: {}
impl Driver for VirtioConsoleDriver { fn get_interrupt_number (& self) -> InterruptLine { self . irq } fn get_name (& self) -> & 'static str { "virtio" } }
};
}
