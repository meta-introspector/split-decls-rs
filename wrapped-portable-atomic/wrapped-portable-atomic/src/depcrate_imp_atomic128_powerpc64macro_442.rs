// Generated macro for macro_442 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_442 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_442"}
// Dependencies: {}
atomic_rmw_ll_sc_3 ! { atomic_umax_pwr8 , [] , "cmpld %r7, {val_lo}" , "iselgt %r9, %r7, {val_lo}" , "cmpld %r6, {val_hi}" , "iselgt %r8, %r7, {val_lo}" , "iseleq %r9, %r9, %r8" , "iselgt %r8, %r6, {val_hi}" , }
};
}
