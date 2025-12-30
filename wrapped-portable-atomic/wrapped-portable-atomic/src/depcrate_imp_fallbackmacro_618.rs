// Generated macro for macro_618 (macro)
macro_rules! Depcrate_imp_fallbackmacro_618 {
() => {
// Module: crate::imp::fallback
// Provides: {"macro_618"}
// Dependencies: {}
# [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (any (test , not (any (not (portable_atomic_no_atomic_64) , all (target_arch = "riscv32" , not (any (miri , portable_atomic_sanitize_thread)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) , any (target_feature = "zacas" , portable_atomic_target_feature = "zacas") ,) ,)))))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (any (test , not (any (target_has_atomic = "64" , all (target_arch = "riscv32" , not (any (miri , portable_atomic_sanitize_thread)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) , any (target_feature = "zacas" , portable_atomic_target_feature = "zacas") ,) ,)))))] cfg_no_fast_atomic_64 ! { atomic ! (AtomicI64 , i64 , 8) ; atomic ! (AtomicU64 , u64 , 8) ; }
};
}
