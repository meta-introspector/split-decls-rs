macro_rules! AssociatedValueExpectedFor {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_associated_value_expected_for)] pub (crate) struct AssociatedValueExpectedFor { # [primary_span] pub span : Span , pub ident : Ident , }
    };
}

AssociatedValueExpectedFor!()