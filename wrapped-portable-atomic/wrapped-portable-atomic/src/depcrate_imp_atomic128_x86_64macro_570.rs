// Generated macro for macro_570 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_570 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_570"}
// Dependencies: {}
atomic_rmw_cas_3 ! { atomic_add_cmpxchg16b , "mov rbx, rax" , "add rbx, rsi" , "mov rcx, rdx" , "adc rcx, r8" , }
};
}
