// Generated macro for impl_206 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsimpl_206 {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"impl_206"}
// Dependencies: {}
impl fmt :: Display for NodeDebugInfo { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (ref name) = self . name { write ! (f , "{tag:?} ({name})" , tag = self . tag) } else { write ! (f , "{tag:?}" , tag = self . tag) } } }
};
}
