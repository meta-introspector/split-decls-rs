macro_rules! deps {
    () => {
        FnCtxt!();
        ConfirmContext!();
        Pick!();
        ConfirmResult!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl < 'a , 'tcx > FnCtxt < 'a , 'tcx > { pub (crate) fn confirm_method (& self , span : Span , self_expr : & 'tcx hir :: Expr < 'tcx > , call_expr : & 'tcx hir :: Expr < 'tcx > , unadjusted_self_ty : Ty < 'tcx > , pick : & probe :: Pick < 'tcx > , segment : & 'tcx hir :: PathSegment < 'tcx > ,) -> ConfirmResult < 'tcx > { debug ! ("confirm(unadjusted_self_ty={:?}, pick={:?}, generic_args={:?})" , unadjusted_self_ty , pick , segment . args ,) ; let mut confirm_cx = ConfirmContext :: new (self , span , self_expr , call_expr) ; confirm_cx . confirm (unadjusted_self_ty , pick , segment) } pub (crate) fn confirm_method_for_diagnostic (& self , span : Span , self_expr : & 'tcx hir :: Expr < 'tcx > , call_expr : & 'tcx hir :: Expr < 'tcx > , unadjusted_self_ty : Ty < 'tcx > , pick : & probe :: Pick < 'tcx > , segment : & hir :: PathSegment < 'tcx > ,) -> ConfirmResult < 'tcx > { let mut confirm_cx = ConfirmContext :: new (self , span , self_expr , call_expr) ; confirm_cx . skip_record_for_diagnostics = true ; confirm_cx . confirm (unadjusted_self_ty , pick , segment) } }
    };
}

impl_246!()