// Generated macro for select_atomic_rmw (macro)
macro_rules! Depcrate_imp_atomic128_x86_64select_atomic_rmw {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"select_atomic_rmw"}
// Dependencies: {}
macro_rules ! select_atomic_rmw { (unsafe fn $ name : ident ($ ($ arg : tt) *) $ (-> $ ret_ty : ty) ?; cmpxchg16b = $ cmpxchg16b_fn : ident ; fallback = $ seqcst_fallback_fn : ident ;) => { # [cfg (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b"))] use self ::$ cmpxchg16b_fn as $ name ; # [cfg (not (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b" ,)))] # [inline] unsafe fn $ name ($ ($ arg) *, _order : Ordering) $ (-> $ ret_ty) ? { fn_alias ! { # [cfg_attr (not (portable_atomic_no_cmpxchg16b_target_feature) , target_feature (enable = "cmpxchg16b"))] unsafe fn ($ ($ arg) *) $ (-> $ ret_ty) ?; cmpxchg16b_seqcst_fn = $ cmpxchg16b_fn (Ordering :: SeqCst) ; } unsafe { ifunc ! (unsafe fn ($ ($ arg) *) $ (-> $ ret_ty) ? { if detect :: detect () . cmpxchg16b () { cmpxchg16b_seqcst_fn } else { fallback ::$ seqcst_fallback_fn } }) } } } ; }
};
}
