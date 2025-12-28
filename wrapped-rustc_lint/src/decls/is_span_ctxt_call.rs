macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! is_span_ctxt_call {
    () => {
        deps!();
        fn is_span_ctxt_call (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) -> bool { match & expr . kind { hir :: ExprKind :: MethodCall (..) => cx . typeck_results () . type_dependent_def_id (expr . hir_id) . is_some_and (| call_did | cx . tcx . is_diagnostic_item (sym :: SpanCtxt , call_did)) , _ => false , } }
    };
}

is_span_ctxt_call!()