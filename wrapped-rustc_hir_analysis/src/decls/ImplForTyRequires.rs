macro_rules! ImplForTyRequires {
    () => {
        # [derive (Subdiagnostic)] # [note (hir_analysis_requires_note)] pub (crate) struct ImplForTyRequires { # [primary_span] pub span : MultiSpan , pub error_predicate : String , pub trait_name : String , pub ty : String , }
    };
}

ImplForTyRequires!();