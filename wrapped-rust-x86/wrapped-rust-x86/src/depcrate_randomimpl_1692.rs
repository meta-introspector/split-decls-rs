// Generated macro for impl_1692 (impl)
macro_rules! Depcrate_randomimpl_1692 {
() => {
// Module: crate::random
// Provides: {"impl_1692"}
// Dependencies: {}
impl RdSeed for u16 { # [doc = " Fills the 16-bit value with a random bit string"] # [doc = ""] # [doc = " # Safety"] # [doc = " Will crash if RDSEED instructions are not supported."] unsafe fn fill_random (& mut self) -> bool { rdseed16 (self) } }
};
}
