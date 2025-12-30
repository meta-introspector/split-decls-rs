// Generated macro for macro_435 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_435 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_435"}
// Dependencies: {}
atomic_rmw_ll_sc_3 ! { atomic_add_pwr8 , [out ("xer") _ ,] , "addc %r9, {val_lo}, %r7" , "adde %r8, {val_hi}, %r6" , }
};
}
