// Generated macro for is_lock_free (function)
macro_rules! Depcrate_imp_atomic64_arm_linuxis_lock_free {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"is_lock_free"}
// Dependencies: {}
# [inline] fn is_lock_free () -> bool { has_kuser_cmpxchg64 () }
};
}
