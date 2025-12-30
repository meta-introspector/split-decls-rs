// Generated macro for pick_our_changes (function)
macro_rules! Depcrate_tree_functionpick_our_changes {
() => {
// Module: crate::tree::function
// Provides: {"pick_our_changes"}
// Dependencies: {}
fn pick_our_changes < 'a > (side : ConflictMapping , ours : & 'a ChangeListRef , theirs : & 'a ChangeListRef ,) -> & 'a ChangeListRef { match side { Original => ours , Swapped => theirs , } }
};
}
