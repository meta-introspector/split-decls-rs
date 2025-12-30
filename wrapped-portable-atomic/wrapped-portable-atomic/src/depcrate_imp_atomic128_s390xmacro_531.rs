// Generated macro for macro_531 (macro)
macro_rules! Depcrate_imp_atomic128_s390xmacro_531 {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"macro_531"}
// Dependencies: {}
atomic_rmw_cas_3 ! { atomic_or , [] , distinct_op ! ("ogr" , "%r13" , "%r1" , "{val_lo}") , distinct_op ! ("ogr" , "%r12" , "%r0" , "{val_hi}") , }
};
}
