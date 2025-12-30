// Generated macro for impl_237 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsimpl_237 {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"impl_237"}
// Dependencies: {}
impl < 'tcx > Tree { # [doc = " Display the contents of the tree."] pub fn print_tree (& self , protected_tags : & FxHashMap < BorTag , ProtectorKind > , show_unnamed : bool ,) -> InterpResult < 'tcx > { let mut indenter = DisplayIndent :: new () ; let ranges = self . locations . iter_all () . map (| (range , _loc) | range) . collect :: < Vec < _ > > () ; if let Some (repr) = DisplayRepr :: from (self , show_unnamed) { repr . print (& DEFAULT_FORMATTER , & mut indenter , protected_tags , ranges , ! show_unnamed ,) ; } interp_ok (()) } }
};
}
