// Generated macro for macro_280 (macro)
macro_rules! Depcrate_imp_atomic64_arm_linuxmacro_280 {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"macro_280"}
// Dependencies: {}
select_atomic ! { unsafe fn atomic_umin (dst : * mut u64 , val : u64) -> u64 { | x | core :: cmp :: min (x , val) } fallback = atomic_umin_seqcst }
};
}
