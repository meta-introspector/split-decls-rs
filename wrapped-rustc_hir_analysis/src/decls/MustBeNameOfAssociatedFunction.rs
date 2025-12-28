macro_rules! MustBeNameOfAssociatedFunction {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_must_be_name_of_associated_function)] pub (crate) struct MustBeNameOfAssociatedFunction { # [primary_span] pub span : Span , }
    };
}

MustBeNameOfAssociatedFunction!();