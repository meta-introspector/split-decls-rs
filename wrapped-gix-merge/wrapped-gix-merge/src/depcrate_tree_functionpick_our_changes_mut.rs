// Generated macro for pick_our_changes_mut (function)
macro_rules! Depcrate_tree_functionpick_our_changes_mut {
() => {
// Module: crate::tree::function
// Provides: {"pick_our_changes_mut"}
// Dependencies: {}
fn pick_our_changes_mut < 'a > (side : ConflictMapping , ours : & 'a mut ChangeList , theirs : & 'a mut ChangeList ,) -> & 'a mut ChangeList { match side { Original => ours , Swapped => theirs , } }
};
}
