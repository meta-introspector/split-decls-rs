// Generated macro for macro_579 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_579 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_579"}
// Dependencies: {}
atomic_rmw_cas_3 ! { atomic_umax_cmpxchg16b , "cmp rsi, rax" , "mov rcx, r8" , "sbb rcx, rdx" , "mov rcx, r8" , "cmovb rcx, rdx" , "mov rbx, rsi" , "cmovb rbx, rax" , }
};
}
