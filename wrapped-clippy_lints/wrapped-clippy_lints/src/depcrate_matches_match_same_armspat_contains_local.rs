// Generated macro for pat_contains_local (function)
macro_rules! Depcrate_matches_match_same_armspat_contains_local {
() => {
// Module: crate::matches::match_same_arms
// Provides: {"pat_contains_local"}
// Dependencies: {}
fn pat_contains_local (pat : & Pat < '_ > , id : HirId) -> bool { let mut result = false ; pat . walk_short (| p | { result |= matches ! (p . kind , PatKind :: Binding (_ , binding_id , ..) if binding_id == id) ; ! result }) ; result }
};
}
