macro_rules! UnionPatDotDot {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_union_pat_dotdot)] pub (crate) struct UnionPatDotDot { # [primary_span] pub span : Span , }
    };
}

UnionPatDotDot!()