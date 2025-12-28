macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! InvalidLiteralValue {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_invalid_literal_value)] pub (crate) struct InvalidLiteralValue { # [primary_span] # [label] pub span : Span , }
    };
}

InvalidLiteralValue!();