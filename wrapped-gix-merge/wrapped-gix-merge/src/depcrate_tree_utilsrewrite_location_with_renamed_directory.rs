// Generated macro for rewrite_location_with_renamed_directory (function)
macro_rules! Depcrate_tree_utilsrewrite_location_with_renamed_directory {
() => {
// Module: crate::tree::utils
// Provides: {"rewrite_location_with_renamed_directory"}
// Dependencies: {}
pub fn rewrite_location_with_renamed_directory (their_location : & BStr , passed_change : & Change) -> Option < BString > { match passed_change { Change :: Rewrite { source_location , location , .. } if passed_change . entry_mode () . is_tree () => { let suffix = their_location . strip_prefix (source_location . as_bytes ()) ? ; let mut rewritten = location . to_owned () ; rewritten . push_str (suffix) ; Some (rewritten) } _ => None , } }
};
}
