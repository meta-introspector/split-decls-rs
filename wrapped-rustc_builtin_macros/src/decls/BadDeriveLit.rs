macro_rules! deps {
    () => {
        BadDeriveLitHelp!();
    };
}

macro_rules! BadDeriveLit {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (builtin_macros_unexpected_lit , code = E0777)] pub (crate) struct BadDeriveLit { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub help : BadDeriveLitHelp , }
    };
}

BadDeriveLit!();