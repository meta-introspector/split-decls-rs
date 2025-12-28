macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! OutOfRangeInteger {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_out_of_range_integer)] pub (crate) struct OutOfRangeInteger { # [primary_span] # [label] pub span : Span , }
    };
}

OutOfRangeInteger!()