// Generated macro for compute_type_match (function)
macro_rules! Depcrate_rendercompute_type_match {
() => {
// Module: crate::render
// Provides: {"compute_type_match"}
// Dependencies: {}
fn compute_type_match (ctx : & CompletionContext < '_ > , completion_ty : & hir :: Type < '_ > ,) -> Option < CompletionRelevanceTypeMatch > { let expected_type = ctx . expected_type . as_ref () ? ; if expected_type . is_unit () { return None ; } match_types (ctx , expected_type , completion_ty) }
};
}
