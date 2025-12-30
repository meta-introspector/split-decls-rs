// Generated macro for impl_386 (impl)
macro_rules! Depcrate_drivers_net_rtl8139impl_386 {
() => {
// Module: crate::drivers::net::rtl8139
// Provides: {"impl_386"}
// Dependencies: {}
impl Drop for RTL8139Driver { fn drop (& mut self) { debug ! ("Dropping RTL8129Driver!") ; self . regs . as_mut_ptr () . cr () . write (CR_RST) ; } }
};
}
