// Generated macro for impl_483 (impl)
macro_rules! Depcrate_infer_snapshot_undo_logimpl_483 {
() => {
// Module: crate::infer::snapshot::undo_log
// Provides: {"impl_483"}
// Dependencies: {}
impl < 'tcx > std :: ops :: IndexMut < usize > for InferCtxtUndoLogs < 'tcx > { fn index_mut (& mut self , key : usize) -> & mut Self :: Output { & mut self . logs [key] } }
};
}
