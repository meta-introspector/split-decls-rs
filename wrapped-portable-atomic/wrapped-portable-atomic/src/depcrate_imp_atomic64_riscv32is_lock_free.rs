// Generated macro for is_lock_free (function)
macro_rules! Depcrate_imp_atomic64_riscv32is_lock_free {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"is_lock_free"}
// Dependencies: {}
# [inline] fn is_lock_free () -> bool { # [cfg (any (target_feature = "zacas" , portable_atomic_target_feature = "zacas"))] { true } # [cfg (not (any (target_feature = "zacas" , portable_atomic_target_feature = "zacas")))] { detect :: detect () . zacas () } }
};
}
