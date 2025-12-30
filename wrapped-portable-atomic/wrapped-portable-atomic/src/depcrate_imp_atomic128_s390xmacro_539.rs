// Generated macro for macro_539 (macro)
macro_rules! Depcrate_imp_atomic128_s390xmacro_539 {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"macro_539"}
// Dependencies: {}
# [cfg (any (target_feature = "distinct-ops" , portable_atomic_target_feature = "distinct-ops"))] atomic_rmw_cas_2 ! { atomic_neg , [zero = in (reg) 0_u64 ,] , "slgrk %r13, {zero}, %r1" , "lghi %r12, 0" , "slbgr %r12, %r0" , }
};
}
