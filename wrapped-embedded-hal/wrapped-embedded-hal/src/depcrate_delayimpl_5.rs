// Generated macro for impl_5 (impl)
macro_rules! Depcrate_delayimpl_5 {
() => {
// Module: crate::delay
// Provides: {"impl_5"}
// Dependencies: {}
impl < T > DelayNs for & mut T where T : DelayNs + ? Sized , { # [inline] fn delay_ns (& mut self , ns : u32) { T :: delay_ns (self , ns) ; } # [inline] fn delay_us (& mut self , us : u32) { T :: delay_us (self , us) ; } # [inline] fn delay_ms (& mut self , ms : u32) { T :: delay_ms (self , ms) ; } }
};
}
