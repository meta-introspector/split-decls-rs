// Generated macro for macro_580 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_580 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_580"}
// Dependencies: {}
atomic_rmw_cas_3 ! { atomic_min_cmpxchg16b , "cmp rsi, rax" , "mov rcx, r8" , "sbb rcx, rdx" , "mov rcx, r8" , "cmovge rcx, rdx" , "mov rbx, rsi" , "cmovge rbx, rax" , }
};
}
