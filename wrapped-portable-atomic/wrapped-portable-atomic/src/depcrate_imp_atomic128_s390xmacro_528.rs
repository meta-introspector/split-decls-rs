// Generated macro for macro_528 (macro)
macro_rules! Depcrate_imp_atomic128_s390xmacro_528 {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"macro_528"}
// Dependencies: {}
atomic_rmw_cas_3 ! { atomic_and , [] , distinct_op ! ("ngr" , "%r13" , "%r1" , "{val_lo}") , distinct_op ! ("ngr" , "%r12" , "%r0" , "{val_hi}") , }
};
}
