// Generated macro for impl_1683 (impl)
macro_rules! Depcrate_randomimpl_1683 {
() => {
// Module: crate::random
// Provides: {"impl_1683"}
// Dependencies: {}
impl RdRand for u16 { # [doc = " Fills the 16-bit value with a random bit string"] # [doc = ""] # [doc = " # Safety"] # [doc = " Will crash if RDRAND instructions are not supported."] unsafe fn fill_random (& mut self) -> bool { rdrand16 (self) } }
};
}
