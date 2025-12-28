macro_rules! deps {
    () => {
        FnCtxt!();
    };
}

macro_rules! ConfirmContext {
    () => {
        deps!();
        struct ConfirmContext < 'a , 'tcx > { fcx : & 'a FnCtxt < 'a , 'tcx > , span : Span , self_expr : & 'tcx hir :: Expr < 'tcx > , call_expr : & 'tcx hir :: Expr < 'tcx > , skip_record_for_diagnostics : bool , }
    };
}

ConfirmContext!()