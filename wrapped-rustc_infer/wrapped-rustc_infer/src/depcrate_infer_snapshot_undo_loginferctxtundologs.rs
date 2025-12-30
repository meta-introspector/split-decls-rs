// Generated macro for InferCtxtUndoLogs (struct)
macro_rules! Depcrate_infer_snapshot_undo_logInferCtxtUndoLogs {
() => {
// Module: crate::infer::snapshot::undo_log
// Provides: {"InferCtxtUndoLogs"}
// Dependencies: {}
# [doc = " The combined undo log for all the various unification tables. For each change to the storage"] # [doc = " for any kind of inference variable, we record an UndoLog entry in the vector here."] # [derive (Clone , Default)] pub (crate) struct InferCtxtUndoLogs < 'tcx > { logs : Vec < UndoLog < 'tcx > > , num_open_snapshots : usize , }
};
}
