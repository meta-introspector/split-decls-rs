macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! FieldAssociatedValueExpected {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_field_associated_value_expected)] pub (crate) struct FieldAssociatedValueExpected { # [primary_span] pub span : Span , pub name : Symbol , }
    };
}

FieldAssociatedValueExpected!();