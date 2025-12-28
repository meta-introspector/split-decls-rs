macro_rules! deps {
    () => {
        FormatUnusedArg!();
    };
}

macro_rules! FormatUnusedArgs {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (builtin_macros_format_unused_args)] pub (crate) struct FormatUnusedArgs { # [primary_span] pub (crate) unused : Vec < Span > , # [label] pub (crate) fmt : Span , # [subdiagnostic] pub (crate) unused_labels : Vec < FormatUnusedArg > , }
    };
}

FormatUnusedArgs!()