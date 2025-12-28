macro_rules! CmseEntryGeneric {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_cmse_entry_generic , code = E0798)] pub (crate) struct CmseEntryGeneric { # [primary_span] pub span : Span , }
    };
}

CmseEntryGeneric!();