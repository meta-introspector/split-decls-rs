macro_rules! AssertRequiresExpression {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_assert_requires_expression)] pub (crate) struct AssertRequiresExpression { # [primary_span] pub (crate) span : Span , # [suggestion (code = "" , applicability = "maybe-incorrect")] pub (crate) token : Span , }
    };
}

AssertRequiresExpression!();