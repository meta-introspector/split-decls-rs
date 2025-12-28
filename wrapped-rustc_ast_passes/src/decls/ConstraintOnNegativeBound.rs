macro_rules! ConstraintOnNegativeBound {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_constraint_on_negative_bound)] pub (crate) struct ConstraintOnNegativeBound { # [primary_span] pub span : Span , }
    };
}

ConstraintOnNegativeBound!()