macro_rules! ManualImplementation {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_manual_implementation , code = E0183)] # [help] pub (crate) struct ManualImplementation { # [primary_span] # [label] pub span : Span , pub trait_name : String , }
    };
}

ManualImplementation!()