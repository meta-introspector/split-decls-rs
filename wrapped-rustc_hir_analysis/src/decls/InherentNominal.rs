macro_rules! InherentNominal {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_inherent_nominal , code = E0118)] # [note] pub (crate) struct InherentNominal { # [primary_span] # [label] pub span : Span , }
    };
}

InherentNominal!()