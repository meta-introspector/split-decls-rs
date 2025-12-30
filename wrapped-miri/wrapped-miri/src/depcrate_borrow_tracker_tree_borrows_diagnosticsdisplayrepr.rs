// Generated macro for DisplayRepr (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsDisplayRepr {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"DisplayRepr"}
// Dependencies: {}
# [doc = " Extracted information from the tree, in a form that is readily accessible"] # [doc = " for printing. I.e. resolve parent-child pointers into an actual tree,"] # [doc = " zip permissions with their tag, remove wrappers, stringify data."] struct DisplayRepr { tag : BorTag , name : Option < String > , rperm : Vec < Option < LocationState > > , children : Vec < DisplayRepr > , }
};
}
