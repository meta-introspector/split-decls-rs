// Generated macro for push_deferred_with_rewrite (function)
macro_rules! Depcrate_tree_functionpush_deferred_with_rewrite {
() => {
// Module: crate::tree::function
// Provides: {"push_deferred_with_rewrite"}
// Dependencies: {}
fn push_deferred_with_rewrite ((change , ours_idx) : (Change , Option < usize >) , new_location : Option < (BString , usize) > , changes : & mut ChangeList ,) { changes . push (TrackedChange { inner : change , was_written : false , needs_tree_insertion : Some (ours_idx) , rewritten_location : new_location , }) ; }
};
}
