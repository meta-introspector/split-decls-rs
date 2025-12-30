// Generated macro for macro_576 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_576 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_576"}
// Dependencies: {}
atomic_rmw_cas_2 ! { atomic_not_cmpxchg16b , "mov rbx, rax" , "not rbx" , "mov rcx, rdx" , "not rcx" , }
};
}
