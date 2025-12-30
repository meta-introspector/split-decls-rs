// Generated macro for macro_527 (macro)
macro_rules! Depcrate_imp_atomic128_s390xmacro_527 {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"macro_527"}
// Dependencies: {}
atomic_rmw_cas_3 ! { atomic_sub , [] , distinct_op ! ("slgr" , "%r13" , "%r1" , "{val_lo}") , "lgr %r12, %r0" , "slbgr %r12, {val_hi}" , }
};
}
