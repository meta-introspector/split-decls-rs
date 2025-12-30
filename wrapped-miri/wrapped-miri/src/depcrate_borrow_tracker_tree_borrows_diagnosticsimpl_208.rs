// Generated macro for impl_208 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsimpl_208 {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"impl_208"}
// Dependencies: {}
impl fmt :: Display for AccessCause { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Explicit (kind) => write ! (f , "{kind}") , Self :: Reborrow => write ! (f , "reborrow") , Self :: Dealloc => write ! (f , "deallocation") , Self :: FnExit (_) => unreachable ! ("protector accesses can never be the source of UB") , } } }
};
}
