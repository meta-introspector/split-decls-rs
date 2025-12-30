// Generated macro for macro_530 (macro)
macro_rules! Depcrate_imp_atomic128_s390xmacro_530 {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"macro_530"}
// Dependencies: {}
# [cfg (not (any (target_feature = "miscellaneous-extensions-3" , portable_atomic_target_feature = "miscellaneous-extensions-3" ,)))] atomic_rmw_cas_3 ! { atomic_nand , [] , distinct_op ! ("ngr" , "%r13" , "%r1" , "{val_lo}") , distinct_op ! ("ngr" , "%r12" , "%r0" , "{val_hi}") , "lcgr %r13, %r13" , "aghi %r13, -1" , "lcgr %r12, %r12" , "aghi %r12, -1" , }
};
}
