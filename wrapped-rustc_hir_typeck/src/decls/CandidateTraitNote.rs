macro_rules! CandidateTraitNote {
    () => {
        # [derive (Subdiagnostic)] # [note (hir_typeck_candidate_trait_note)] pub (crate) struct CandidateTraitNote { # [primary_span] pub span : Span , pub trait_name : String , pub item_name : Ident , pub action_or_ty : String , }
    };
}

CandidateTraitNote!()