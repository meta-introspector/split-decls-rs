// Generated macro for is_lock_free (function)
macro_rules! Depcrate_imp_atomic128_powerpc64is_lock_free {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"is_lock_free"}
// Dependencies: {}
# [inline] fn is_lock_free () -> bool { # [cfg (any (target_feature = "quadword-atomics" , portable_atomic_target_feature = "quadword-atomics" ,))] { true } # [cfg (not (any (target_feature = "quadword-atomics" , portable_atomic_target_feature = "quadword-atomics" ,)))] { detect :: detect () . quadword_atomics () } }
};
}
