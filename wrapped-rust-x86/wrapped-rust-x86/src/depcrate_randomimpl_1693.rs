// Generated macro for impl_1693 (impl)
macro_rules! Depcrate_randomimpl_1693 {
() => {
// Module: crate::random
// Provides: {"impl_1693"}
// Dependencies: {}
impl RdSeed for u32 { # [doc = " Fills the 32-bit value with a random bit string"] # [doc = ""] # [doc = " # Safety"] # [doc = " Will crash if RDSEED instructions are not supported."] unsafe fn fill_random (& mut self) -> bool { rdseed32 (self) } }
};
}
