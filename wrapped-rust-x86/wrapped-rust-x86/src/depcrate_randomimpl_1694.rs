// Generated macro for impl_1694 (impl)
macro_rules! Depcrate_randomimpl_1694 {
() => {
// Module: crate::random
// Provides: {"impl_1694"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] impl RdSeed for u64 { # [doc = " Fills the 64-bit value with a random bit string"] # [doc = ""] # [doc = " # Safety"] # [doc = " Will crash if RDSEED instructions are not supported."] unsafe fn fill_random (& mut self) -> bool { rdseed64 (self) } }
};
}
