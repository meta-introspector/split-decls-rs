// Generated macro for impl_101 (impl)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_diagnosticsimpl_101 {
() => {
// Module: crate::borrow_tracker::stacked_borrows::diagnostics
// Provides: {"impl_101"}
// Dependencies: {}
impl fmt :: Display for InvalidationCause { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { InvalidationCause :: Access (kind) => write ! (f , "{kind}") , InvalidationCause :: Retag (perm , info) => write ! (f , "{perm:?} {retag}" , retag = info . summary ()) , } } }
};
}
