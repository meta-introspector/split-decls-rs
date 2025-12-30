// Generated macro for impl_1684 (impl)
macro_rules! Depcrate_randomimpl_1684 {
() => {
// Module: crate::random
// Provides: {"impl_1684"}
// Dependencies: {}
impl RdRand for u32 { # [doc = " Fills the 32-bit value with a random bit string"] # [doc = ""] # [doc = " # Safety"] # [doc = " Will crash if RDRAND instructions are not supported."] unsafe fn fill_random (& mut self) -> bool { rdrand32 (self) } }
};
}
