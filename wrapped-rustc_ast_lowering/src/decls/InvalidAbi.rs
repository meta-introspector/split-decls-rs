macro_rules! deps {
    () => {
        InvalidAbiSuggestion!();
    };
}

macro_rules! InvalidAbi {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (ast_lowering_invalid_abi , code = E0703)] # [note] pub (crate) struct InvalidAbi { # [primary_span] # [label] pub span : Span , pub abi : Symbol , pub command : String , # [subdiagnostic] pub suggestion : Option < InvalidAbiSuggestion > , }
    };
}

InvalidAbi!()