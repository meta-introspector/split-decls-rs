macro_rules! DeferredCallResolution {
    () => {
        # [derive (Debug)] pub (crate) struct DeferredCallResolution < 'tcx > { call_expr : & 'tcx hir :: Expr < 'tcx > , callee_expr : & 'tcx hir :: Expr < 'tcx > , closure_ty : Ty < 'tcx > , adjustments : Vec < Adjustment < 'tcx > > , fn_sig : ty :: FnSig < 'tcx > , }
    };
}

DeferredCallResolution!()