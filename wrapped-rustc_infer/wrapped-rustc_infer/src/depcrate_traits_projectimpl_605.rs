// Generated macro for impl_605 (impl)
macro_rules! Depcrate_traits_projectimpl_605 {
() => {
// Module: crate::traits::project
// Provides: {"impl_605"}
// Dependencies: {}
impl < 'tcx > ProjectionCacheStorage < 'tcx > { # [inline] pub (crate) fn with_log < 'a > (& 'a mut self , undo_log : & 'a mut InferCtxtUndoLogs < 'tcx > ,) -> ProjectionCache < 'a , 'tcx > { ProjectionCache { map : & mut self . map , undo_log } } }
};
}
