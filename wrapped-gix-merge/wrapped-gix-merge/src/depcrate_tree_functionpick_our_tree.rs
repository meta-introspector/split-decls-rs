// Generated macro for pick_our_tree (function)
macro_rules! Depcrate_tree_functionpick_our_tree {
() => {
// Module: crate::tree::function
// Provides: {"pick_our_tree"}
// Dependencies: {}
fn pick_our_tree < 'a > (side : ConflictMapping , ours : & 'a mut TreeNodes , theirs : & 'a mut TreeNodes) -> & 'a mut TreeNodes { match side { Original => ours , Swapped => theirs , } }
};
}
