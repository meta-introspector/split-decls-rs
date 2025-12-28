macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! NoField {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_no_field)] pub (crate) struct NoField { # [primary_span] pub span : Span , pub name : Symbol , }
    };
}

NoField!();