// Generated macro for macro_441 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_441 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_441"}
// Dependencies: {}
atomic_rmw_ll_sc_3 ! { atomic_max_pwr8 , [out ("cr1") _ ,] , "cmpld %r7, {val_lo}" , "iselgt %r9, %r7, {val_lo}" , "cmpd %cr1, %r6, {val_hi}" , "isel %r8, %r7, {val_lo}, 5" , "cmpld %r6, {val_hi}" , "iseleq %r9, %r9, %r8" , "isel %r8, %r6, {val_hi}, 5" , }
};
}
