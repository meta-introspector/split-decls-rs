// Generated macro for IS_ALWAYS_LOCK_FREE (const)
macro_rules! Depcrate_imp_atomic128_riscv64IS_ALWAYS_LOCK_FREE {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"IS_ALWAYS_LOCK_FREE"}
// Dependencies: {}
const IS_ALWAYS_LOCK_FREE : bool = cfg ! (any (target_feature = "zacas" , portable_atomic_target_feature = "zacas")) ;
};
}
