// Generated macro for macro_572 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_572 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_572"}
// Dependencies: {}
atomic_rmw_cas_3 ! { atomic_and_cmpxchg16b , "mov rbx, rax" , "and rbx, rsi" , "mov rcx, rdx" , "and rcx, r8" , }
};
}
