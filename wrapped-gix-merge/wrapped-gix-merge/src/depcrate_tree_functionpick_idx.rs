// Generated macro for pick_idx (function)
macro_rules! Depcrate_tree_functionpick_idx {
() => {
// Module: crate::tree::function
// Provides: {"pick_idx"}
// Dependencies: {}
fn pick_idx (side : ConflictMapping , ours : usize , theirs : usize) -> usize { match side { Original => ours , Swapped => theirs , } }
};
}
