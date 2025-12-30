// Generated macro for select_op (macro)
macro_rules! Depcrate_imp_atomic128_s390xselect_op {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"select_op"}
// Dependencies: {}
# [cfg (not (any (target_feature = "miscellaneous-extensions-3" , portable_atomic_target_feature = "miscellaneous-extensions-3" ,)))] # [cfg (any (target_feature = "load-store-on-cond" , portable_atomic_target_feature = "load-store-on-cond" ,))] macro_rules ! select_op { ($ cond : tt , $ a0 : tt , $ a1 : tt , $ a2 : tt) => { concat ! ("lgr " , $ a0 , ", " , $ a2 , "\n" , "locgr" , $ cond , " " , $ a0 , ", " , $ a1) } ; }
};
}
