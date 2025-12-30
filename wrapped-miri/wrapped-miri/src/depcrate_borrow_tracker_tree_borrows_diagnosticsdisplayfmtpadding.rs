// Generated macro for DisplayFmtPadding (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsDisplayFmtPadding {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"DisplayFmtPadding"}
// Dependencies: {}
# [doc = " Formatting of the tree structure."] # [doc = ""] # [doc = " Example:"] # [doc = " ```rust,ignore (private type)"] # [doc = " DisplayFmtPadding {"] # [doc = "     join_middle: \"|-\","] # [doc = "     join_last: \"'-\","] # [doc = "     join_haschild: \"-+-\","] # [doc = "     join_default: \"---\","] # [doc = "     indent_middle: \"| \","] # [doc = "     indent_last: \"  \","] # [doc = " }"] # [doc = " ```"] # [doc = " will show the tree as"] # [doc = " ```text"] # [doc = " -+- root"] # [doc = "  |--+- a"] # [doc = "  |  '--+- b"] # [doc = "  |     '---- c"] # [doc = "  |--+- d"] # [doc = "  |  '---- e"] # [doc = "  '---- f"] # [doc = " ```"] struct DisplayFmtPadding { # [doc = " Connector for a child other than the last."] join_middle : S , # [doc = " Connector for the last child. Should have the same width as `join_middle`."] join_last : S , # [doc = " Connector for a node that itself has a child."] join_haschild : S , # [doc = " Connector for a node that does not have a child. Should have the same width"] # [doc = " as `join_haschild`."] join_default : S , # [doc = " Indentation when there is a next child."] indent_middle : S , # [doc = " Indentation for the last child."] indent_last : S , }
};
}
