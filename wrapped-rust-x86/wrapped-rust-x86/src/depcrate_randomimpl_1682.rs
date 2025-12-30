// Generated macro for impl_1682 (impl)
macro_rules! Depcrate_randomimpl_1682 {
() => {
// Module: crate::random
// Provides: {"impl_1682"}
// Dependencies: {}
impl RdRand for u8 { # [doc = " Fills the 16-bit value with a random bit string"] # [doc = ""] # [doc = " # Safety"] # [doc = " Will crash if RDSEED instructions are not supported."] unsafe fn fill_random (& mut self) -> bool { let mut r : u16 = 0 ; let ret = rdrand16 (& mut r) ; * self = r as u8 ; ret } }
};
}
