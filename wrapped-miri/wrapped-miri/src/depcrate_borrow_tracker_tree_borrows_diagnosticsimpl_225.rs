// Generated macro for impl_225 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsimpl_225 {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"impl_225"}
// Dependencies: {}
impl < 'tcx > Tree { # [doc = " Display the contents of the tree."] pub fn print_tree (& self , protected_tags : & FxHashMap < BorTag , ProtectorKind > , show_unnamed : bool ,) -> InterpResult < 'tcx > { let mut indenter = DisplayIndent :: new () ; let ranges = self . rperms . iter_all () . map (| (range , _perms) | range) . collect :: < Vec < _ > > () ; if let Some (repr) = DisplayRepr :: from (self , show_unnamed) { repr . print (& DEFAULT_FORMATTER , & mut indenter , protected_tags , ranges , ! show_unnamed ,) ; } interp_ok (()) } }
};
}
