// Generated macro for macro_278 (macro)
macro_rules! Depcrate_imp_atomic64_arm_linuxmacro_278 {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"macro_278"}
// Dependencies: {}
select_atomic ! { unsafe fn atomic_umax (dst : * mut u64 , val : u64) -> u64 { | x | core :: cmp :: max (x , val) } fallback = atomic_umax_seqcst }
};
}
