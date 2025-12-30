// Generated macro for IS_ALWAYS_LOCK_FREE (const)
macro_rules! Depcrate_imp_atomic64_riscv32IS_ALWAYS_LOCK_FREE {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"IS_ALWAYS_LOCK_FREE"}
// Dependencies: {}
const IS_ALWAYS_LOCK_FREE : bool = cfg ! (any (target_feature = "zacas" , portable_atomic_target_feature = "zacas")) ;
};
}
