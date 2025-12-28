macro_rules! MustImplementOneOfAttribute {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_must_implement_one_of_attribute)] pub (crate) struct MustImplementOneOfAttribute { # [primary_span] pub span : Span , }
    };
}

MustImplementOneOfAttribute!()