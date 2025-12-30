// Generated macro for macro_573 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_573 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_573"}
// Dependencies: {}
atomic_rmw_cas_3 ! { atomic_nand_cmpxchg16b , "mov rbx, rax" , "and rbx, rsi" , "not rbx" , "mov rcx, rdx" , "and rcx, r8" , "not rcx" , }
};
}
