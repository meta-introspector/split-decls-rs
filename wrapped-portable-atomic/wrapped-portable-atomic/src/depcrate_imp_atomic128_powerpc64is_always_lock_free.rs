// Generated macro for IS_ALWAYS_LOCK_FREE (const)
macro_rules! Depcrate_imp_atomic128_powerpc64IS_ALWAYS_LOCK_FREE {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"IS_ALWAYS_LOCK_FREE"}
// Dependencies: {}
const IS_ALWAYS_LOCK_FREE : bool = cfg ! (any (target_feature = "quadword-atomics" , portable_atomic_target_feature = "quadword-atomics" ,)) ;
};
}
