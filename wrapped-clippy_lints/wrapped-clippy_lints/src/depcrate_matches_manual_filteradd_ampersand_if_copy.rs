// Generated macro for add_ampersand_if_copy (function)
macro_rules! Depcrate_matches_manual_filteradd_ampersand_if_copy {
() => {
// Module: crate::matches::manual_filter
// Provides: {"add_ampersand_if_copy"}
// Dependencies: {}
fn add_ampersand_if_copy (body_str : String , has_copy_trait : bool) -> String { if has_copy_trait { let mut with_ampersand = body_str ; with_ampersand . insert (1 , '&') ; with_ampersand } else { body_str } }
};
}
