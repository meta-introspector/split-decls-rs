// Generated macro for apply_our_resolution (function)
macro_rules! Depcrate_tree_functionapply_our_resolution {
() => {
// Module: crate::tree::function
// Provides: {"apply_our_resolution"}
// Dependencies: {}
fn apply_our_resolution (local_ours : & Change , local_theirs : & Change , outer_side : ConflictMapping , editor : & mut gix_object :: tree :: Editor < '_ > ,) -> Result < () , Error > { let ours = match outer_side { Original => local_ours , Swapped => local_theirs , } ; Ok (apply_change (editor , ours , None) ?) }
};
}
