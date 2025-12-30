// Generated macro for macro_574 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_574 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_574"}
// Dependencies: {}
atomic_rmw_cas_3 ! { atomic_or_cmpxchg16b , "mov rbx, rax" , "or rbx, rsi" , "mov rcx, rdx" , "or rcx, r8" , }
};
}
