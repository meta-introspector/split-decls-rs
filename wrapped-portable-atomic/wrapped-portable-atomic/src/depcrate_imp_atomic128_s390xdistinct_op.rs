// Generated macro for distinct_op (macro)
macro_rules! Depcrate_imp_atomic128_s390xdistinct_op {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"distinct_op"}
// Dependencies: {}
# [cfg (not (any (target_feature = "distinct-ops" , portable_atomic_target_feature = "distinct-ops")))] macro_rules ! distinct_op { ($ op : tt , $ a0 : tt , $ a1 : tt , $ a2 : tt) => { concat ! ("lgr " , $ a0 , ", " , $ a1 , "\n" , $ op , " " , $ a0 , ", " , $ a2) } ; }
};
}
