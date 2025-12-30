// Generated macro for macro_266 (macro)
macro_rules! Depcrate_imp_atomic64_arm_linuxmacro_266 {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"macro_266"}
// Dependencies: {}
select_atomic ! { unsafe fn atomic_load (src : * mut u64) -> u64 { | old | old } fallback = atomic_load_seqcst }
};
}
