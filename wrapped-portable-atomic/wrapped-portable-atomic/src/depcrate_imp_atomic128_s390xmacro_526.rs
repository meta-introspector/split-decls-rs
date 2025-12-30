// Generated macro for macro_526 (macro)
macro_rules! Depcrate_imp_atomic128_s390xmacro_526 {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"macro_526"}
// Dependencies: {}
atomic_rmw_cas_3 ! { atomic_add , [] , distinct_op ! ("algr" , "%r13" , "%r1" , "{val_lo}") , "lgr %r12, %r0" , "alcgr %r12, {val_hi}" , }
};
}
