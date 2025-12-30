// Generated macro for TreeVisitorStack (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_treeTreeVisitorStack {
() => {
// Module: crate::borrow_tracker::tree_borrows::tree
// Provides: {"TreeVisitorStack"}
// Dependencies: {}
# [doc = " Stack of nodes left to explore in a tree traversal."] # [doc = " See the docs of `traverse_this_parents_children_other` for details on the"] # [doc = " traversal order."] struct TreeVisitorStack < NodeContinue , NodeApp , ErrHandler > { # [doc = " Identifier of the original access."] initial : UniIndex , # [doc = " Function describing whether to continue at a tag."] # [doc = " This is only invoked for foreign accesses."] f_continue : NodeContinue , # [doc = " Function to apply to each tag."] f_propagate : NodeApp , # [doc = " Handler to add the required context to diagnostics."] err_builder : ErrHandler , # [doc = " Mutable state of the visit: the tags left to handle."] # [doc = " Every tag pushed should eventually be handled,"] # [doc = " and the precise order is relevant for diagnostics."] # [doc = " Since the traversal is piecewise bottom-up, we need to"] # [doc = " remember whether we're here initially, or after visiting all children."] # [doc = " The last element indicates this."] # [doc = " This is just an artifact of how you hand-roll recursion,"] # [doc = " it does not have a deeper meaning otherwise."] stack : Vec < (UniIndex , AccessRelatedness , RecursionState) > , }
};
}
