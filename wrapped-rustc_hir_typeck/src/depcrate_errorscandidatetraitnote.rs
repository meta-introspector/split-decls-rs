// Generated macro for CandidateTraitNote (struct)
macro_rules! Depcrate_errorsCandidateTraitNote {
() => {
// Module: crate::errors
// Provides: {"CandidateTraitNote"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [note (hir_typeck_candidate_trait_note)] pub (crate) struct CandidateTraitNote { # [primary_span] pub span : Span , pub trait_name : String , pub item_name : Ident , pub action_or_ty : String , }
};
}
