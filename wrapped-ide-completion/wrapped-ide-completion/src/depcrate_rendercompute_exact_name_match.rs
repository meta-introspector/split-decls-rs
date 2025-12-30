// Generated macro for compute_exact_name_match (function)
macro_rules! Depcrate_rendercompute_exact_name_match {
() => {
// Module: crate::render
// Provides: {"compute_exact_name_match"}
// Dependencies: {}
fn compute_exact_name_match (ctx : & CompletionContext < '_ > , completion_name : & str) -> bool { ctx . expected_name . as_ref () . is_some_and (| name | name . text () == completion_name) }
};
}
