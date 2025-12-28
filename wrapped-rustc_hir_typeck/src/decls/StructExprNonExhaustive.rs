macro_rules! StructExprNonExhaustive {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_struct_expr_non_exhaustive , code = E0639)] pub (crate) struct StructExprNonExhaustive { # [primary_span] pub span : Span , pub what : & 'static str , }
    };
}

StructExprNonExhaustive!();