// Generated macro for macro_577 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64macro_577 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"macro_577"}
// Dependencies: {}
atomic_rmw_cas_2 ! { atomic_neg_cmpxchg16b , "mov rbx, rax" , "neg rbx" , "mov rcx, 0" , "sbb rcx, rdx" , }
};
}
