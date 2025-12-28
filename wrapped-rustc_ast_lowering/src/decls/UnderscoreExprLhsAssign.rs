macro_rules! UnderscoreExprLhsAssign {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_underscore_expr_lhs_assign)] pub (crate) struct UnderscoreExprLhsAssign { # [primary_span] # [label] pub span : Span , }
    };
}

UnderscoreExprLhsAssign!()