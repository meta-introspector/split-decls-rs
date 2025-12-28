macro_rules! CmseCallGeneric {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_cmse_call_generic , code = E0798)] pub (crate) struct CmseCallGeneric { # [primary_span] pub span : Span , }
    };
}

CmseCallGeneric!()