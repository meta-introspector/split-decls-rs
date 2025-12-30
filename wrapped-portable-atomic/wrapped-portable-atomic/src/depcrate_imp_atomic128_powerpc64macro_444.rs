// Generated macro for macro_444 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64macro_444 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"macro_444"}
// Dependencies: {}
atomic_rmw_ll_sc_3 ! { atomic_umin_pwr8 , [] , "cmpld %r7, {val_lo}" , "isellt %r9, %r7, {val_lo}" , "cmpld %r6, {val_hi}" , "isellt %r8, %r7, {val_lo}" , "iseleq %r9, %r9, %r8" , "isellt %r8, %r6, {val_hi}" , }
};
}
