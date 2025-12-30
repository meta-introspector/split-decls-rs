// Generated macro for impl_1685 (impl)
macro_rules! Depcrate_randomimpl_1685 {
() => {
// Module: crate::random
// Provides: {"impl_1685"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] impl RdRand for u64 { # [doc = " Fills the 64-bit value with a random bit string"] # [doc = ""] # [doc = " # Safety"] # [doc = " Will crash if RDRAND instructions are not supported."] unsafe fn fill_random (& mut self) -> bool { rdrand64 (self) } }
};
}
