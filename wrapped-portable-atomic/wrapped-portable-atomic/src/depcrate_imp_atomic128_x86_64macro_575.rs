// Generated macro for macro_575 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_575 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_575"}
// Dependencies: {}
atomic_rmw_cas_3 ! { atomic_xor_cmpxchg16b , "mov rbx, rax" , "xor rbx, rsi" , "mov rcx, rdx" , "xor rcx, r8" , }
};
}
