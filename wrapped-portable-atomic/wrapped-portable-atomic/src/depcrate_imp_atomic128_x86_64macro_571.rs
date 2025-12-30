// Generated macro for macro_571 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_571 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_571"}
// Dependencies: {}
atomic_rmw_cas_3 ! { atomic_sub_cmpxchg16b , "mov rbx, rax" , "sub rbx, rsi" , "mov rcx, rdx" , "sbb rcx, r8" , }
};
}
