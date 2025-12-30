// Generated macro for macro_578 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_578 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_578"}
// Dependencies: {}
atomic_rmw_cas_3 ! { atomic_max_cmpxchg16b , "cmp rsi, rax" , "mov rcx, r8" , "sbb rcx, rdx" , "mov rcx, r8" , "cmovl rcx, rdx" , "mov rbx, rsi" , "cmovl rbx, rax" , }
};
}
