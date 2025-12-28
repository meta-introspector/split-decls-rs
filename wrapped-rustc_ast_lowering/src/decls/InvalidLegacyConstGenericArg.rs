macro_rules! deps {
    () => {
        UseConstGenericArg!();
    };
}

macro_rules! InvalidLegacyConstGenericArg {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (ast_lowering_invalid_legacy_const_generic_arg)] pub (crate) struct InvalidLegacyConstGenericArg { # [primary_span] pub span : Span , # [subdiagnostic] pub suggestion : UseConstGenericArg , }
    };
}

InvalidLegacyConstGenericArg!();