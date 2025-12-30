// Generated macro for riscv64 (module)
macro_rules! Depcrate_imp_atomic128riscv64 {
() => {
// Module: crate::imp::atomic128
// Provides: {"riscv64"}
// Dependencies: {}
# [cfg (all (target_arch = "riscv64" , not (any (miri , portable_atomic_sanitize_thread)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) , any (target_feature = "zacas" , portable_atomic_target_feature = "zacas" , all (feature = "fallback" , not (portable_atomic_no_outline_atomics) , any (target_os = "linux" , target_os = "android") ,) ,) ,))] pub (super) mod riscv64 ;
};
}
