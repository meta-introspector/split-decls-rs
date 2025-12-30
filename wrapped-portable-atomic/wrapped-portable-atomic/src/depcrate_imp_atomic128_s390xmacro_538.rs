// Generated macro for macro_538 (macro)
macro_rules! Depcrate_imp_atomic128_s390xmacro_538 {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"macro_538"}
// Dependencies: {}
atomic_rmw_cas_2 ! { atomic_not , [] , "lcgr %r13, %r1" , "aghi %r13, -1" , "lcgr %r12, %r0" , "aghi %r12, -1" , }
};
}
