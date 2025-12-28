macro_rules! deps {
    () => {
        LateContext!();
        SpanUseEqCtxtDiag!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for SpanUseEqCtxt { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & hir :: Expr < '_ >) { if let hir :: ExprKind :: Binary (hir :: BinOp { node : hir :: BinOpKind :: Eq | hir :: BinOpKind :: Ne , .. } , lhs , rhs ,) = expr . kind { if is_span_ctxt_call (cx , lhs) && is_span_ctxt_call (cx , rhs) { cx . emit_span_lint (SPAN_USE_EQ_CTXT , expr . span , SpanUseEqCtxtDiag) ; } } } }
    };
}

impl_298!();