macro_rules! InherentDyn {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_inherent_dyn , code = E0785)] # [note] pub (crate) struct InherentDyn { # [primary_span] # [label] pub span : Span , }
    };
}

InherentDyn!();