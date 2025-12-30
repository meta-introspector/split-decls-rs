// Generated macro for DEFAULT_FORMATTER (const)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsDEFAULT_FORMATTER {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"DEFAULT_FORMATTER"}
// Dependencies: {}
const DEFAULT_FORMATTER : DisplayFmt = DisplayFmt { wrapper : DisplayFmtWrapper { top : '─' , bot : '─' , warning_text : "Warning: this tree is indicative only. Some tags may have been hidden." , } , perm : DisplayFmtPermission { open : "|" , sep : "|" , close : "|" , uninit : "----" , range_sep : ".." } , padding : DisplayFmtPadding { join_middle : "├" , join_last : "└" , indent_middle : "│ " , indent_last : "  " , join_haschild : "┬" , join_default : "─" , } , accessed : DisplayFmtAccess { yes : " " , no : "?" , meh : "-" } , } ;
};
}
