macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! ExpectedNameValuePair {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_expected_name_value_pair)] pub (crate) struct ExpectedNameValuePair { # [primary_span] pub span : Span , }
    };
}

ExpectedNameValuePair!();