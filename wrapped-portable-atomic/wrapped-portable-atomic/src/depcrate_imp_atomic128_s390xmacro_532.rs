// Generated macro for macro_532 (macro)
macro_rules! Depcrate_imp_atomic128_s390xmacro_532 {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"macro_532"}
// Dependencies: {}
atomic_rmw_cas_3 ! { atomic_xor , [] , distinct_op ! ("xgr" , "%r13" , "%r1" , "{val_lo}") , distinct_op ! ("xgr" , "%r12" , "%r0" , "{val_hi}") , }
};
}
