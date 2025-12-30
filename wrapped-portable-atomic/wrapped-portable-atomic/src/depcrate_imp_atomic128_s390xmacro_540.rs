// Generated macro for macro_540 (macro)
macro_rules! Depcrate_imp_atomic128_s390xmacro_540 {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"macro_540"}
// Dependencies: {}
# [cfg (not (any (target_feature = "distinct-ops" , portable_atomic_target_feature = "distinct-ops")))] atomic_rmw_cas_2 ! { atomic_neg , [] , "lghi %r13, 0" , "slgr %r13, %r1" , "lghi %r12, 0" , "slbgr %r12, %r0" , }
};
}
