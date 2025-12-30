// Generated macro for impl_268 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_treeimpl_268 {
() => {
// Module: crate::borrow_tracker::tree_borrows::tree
// Provides: {"impl_268"}
// Dependencies: {}
impl fmt :: Display for LocationState { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . permission) ? ; if ! self . accessed { write ! (f , "?") ? ; } Ok (()) } }
};
}
