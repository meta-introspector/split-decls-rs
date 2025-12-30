// Generated macro for macro_533 (macro)
macro_rules! Depcrate_imp_atomic128_s390xmacro_533 {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"macro_533"}
// Dependencies: {}
# [cfg (any (target_feature = "load-store-on-cond" , portable_atomic_target_feature = "load-store-on-cond" ,))] atomic_rmw_cas_3 ! { atomic_max , [] , "clgr %r1, {val_lo}" , select_op ! ("h" , "%r12" , "%r1" , "{val_lo}") , "cgr %r0, {val_hi}" , select_op ! ("h" , "%r13" , "%r1" , "{val_lo}") , "locgre %r13, %r12" , select_op ! ("h" , "%r12" , "%r0" , "{val_hi}") , }
};
}
