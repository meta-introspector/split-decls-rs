// Generated macro for is_lock_free (function)
macro_rules! Depcrate_imp_atomic128_x86_64is_lock_free {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"is_lock_free"}
// Dependencies: {}
# [inline] fn is_lock_free () -> bool { # [cfg (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b"))] { true } # [cfg (not (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b")))] { detect :: detect () . cmpxchg16b () } }
};
}
