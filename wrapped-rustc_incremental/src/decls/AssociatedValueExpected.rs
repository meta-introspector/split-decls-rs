macro_rules! AssociatedValueExpected {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_associated_value_expected)] pub (crate) struct AssociatedValueExpected { # [primary_span] pub span : Span , }
    };
}

AssociatedValueExpected!()