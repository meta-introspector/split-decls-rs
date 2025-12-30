// Generated macro for impl_607 (impl)
macro_rules! Depcrate_traits_projectimpl_607 {
() => {
// Module: crate::traits::project
// Provides: {"impl_607"}
// Dependencies: {}
impl < 'tcx > Rollback < UndoLog < 'tcx > > for ProjectionCacheStorage < 'tcx > { fn reverse (& mut self , undo : UndoLog < 'tcx >) { self . map . reverse (undo) ; } }
};
}
