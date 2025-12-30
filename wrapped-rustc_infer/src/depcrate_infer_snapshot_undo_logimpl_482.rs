// Generated macro for impl_482 (impl)
macro_rules! Depcrate_infer_snapshot_undo_logimpl_482 {
() => {
// Module: crate::infer::snapshot::undo_log
// Provides: {"impl_482"}
// Dependencies: {}
impl < 'tcx > std :: ops :: Index < usize > for InferCtxtUndoLogs < 'tcx > { type Output = UndoLog < 'tcx > ; fn index (& self , key : usize) -> & Self :: Output { & self . logs [key] } }
};
}
