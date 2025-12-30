// Generated macro for impl_3 (impl)
macro_rules! Depcrate_delayimpl_3 {
() => {
// Module: crate::delay
// Provides: {"impl_3"}
// Dependencies: {}
impl < T > DelayNs for & mut T where T : DelayNs + ? Sized , { # [inline] async fn delay_ns (& mut self , ns : u32) { T :: delay_ns (self , ns) . await ; } # [inline] async fn delay_us (& mut self , us : u32) { T :: delay_us (self , us) . await ; } # [inline] async fn delay_ms (& mut self , ms : u32) { T :: delay_ms (self , ms) . await ; } }
};
}
