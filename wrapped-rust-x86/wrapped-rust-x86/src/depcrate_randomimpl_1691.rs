// Generated macro for impl_1691 (impl)
macro_rules! Depcrate_randomimpl_1691 {
() => {
// Module: crate::random
// Provides: {"impl_1691"}
// Dependencies: {}
impl RdSeed for u8 { # [doc = " Fills the 16-bit value with a random bit string"] # [doc = ""] # [doc = " # Safety"] # [doc = " Will crash if RDSEED instructions are not supported."] unsafe fn fill_random (& mut self) -> bool { let mut r : u16 = 0 ; let ret = rdseed16 (& mut r) ; * self = r as u8 ; ret } }
};
}
