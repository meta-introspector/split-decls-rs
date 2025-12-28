macro_rules! AssertRequiresBoolean {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_assert_requires_boolean)] pub (crate) struct AssertRequiresBoolean { # [primary_span] # [label] pub (crate) span : Span , }
    };
}

AssertRequiresBoolean!();