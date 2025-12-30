// Generated macro for check_arms (function)
macro_rules! Depcrate_manual_option_as_slicecheck_arms {
() => {
// Module: crate::manual_option_as_slice
// Provides: {"check_arms"}
// Dependencies: {}
fn check_arms (cx : & LateContext < '_ > , none_arm : & Arm < '_ > , some_arm : & Arm < '_ >) -> bool { if none_arm . guard . is_none () && some_arm . guard . is_none () && is_empty_slice (cx , none_arm . body) && let Some (name) = extract_ident_from_some_pat (cx , some_arm . pat) { check_some_body (cx , name , some_arm . body) } else { false } }
};
}
