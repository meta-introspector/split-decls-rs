// Generated macro for macro_529 (macro)
macro_rules! Depcrate_imp_atomic128_s390xmacro_529 {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"macro_529"}
// Dependencies: {}
# [cfg (any (target_feature = "miscellaneous-extensions-3" , portable_atomic_target_feature = "miscellaneous-extensions-3" ,))] atomic_rmw_cas_3 ! { atomic_nand , [] , "nngrk %r13, %r1, {val_lo}" , "nngrk %r12, %r0, {val_hi}" , }
};
}
