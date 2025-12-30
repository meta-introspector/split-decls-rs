// Generated macro for macro_581 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_581 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_581"}
// Dependencies: {}
atomic_rmw_cas_3 ! { atomic_umin_cmpxchg16b , "cmp rsi, rax" , "mov rcx, r8" , "sbb rcx, rdx" , "mov rcx, r8" , "cmovae rcx, rdx" , "mov rbx, rsi" , "cmovae rbx, rax" , }
};
}
