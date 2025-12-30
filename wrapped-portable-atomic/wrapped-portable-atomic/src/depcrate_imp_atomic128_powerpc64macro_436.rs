// Generated macro for macro_436 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_436 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_436"}
// Dependencies: {}
atomic_rmw_ll_sc_3 ! { atomic_sub_pwr8 , [out ("xer") _ ,] , "subc %r9, %r7, {val_lo}" , "subfe %r8, {val_hi}, %r6" , }
};
}
