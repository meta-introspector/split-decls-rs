// Generated macro for impl_480 (impl)
macro_rules! Depcrate_infer_snapshot_undo_logimpl_480 {
() => {
// Module: crate::infer::snapshot::undo_log
// Provides: {"impl_480"}
// Dependencies: {}
impl < 'tcx > InferCtxtInner < 'tcx > { pub fn rollback_to (& mut self , snapshot : Snapshot < 'tcx >) { debug ! ("rollback_to({})" , snapshot . undo_len) ; self . undo_log . assert_open_snapshot (& snapshot) ; while self . undo_log . logs . len () > snapshot . undo_len { let undo = self . undo_log . logs . pop () . unwrap () ; self . reverse (undo) ; } self . type_variable_storage . finalize_rollback () ; if self . undo_log . num_open_snapshots == 1 { assert ! (snapshot . undo_len == 0) ; assert ! (self . undo_log . logs . is_empty ()) ; } self . undo_log . num_open_snapshots -= 1 ; } pub fn commit (& mut self , snapshot : Snapshot < 'tcx >) { debug ! ("commit({})" , snapshot . undo_len) ; if self . undo_log . num_open_snapshots == 1 { assert ! (snapshot . undo_len == 0) ; self . undo_log . logs . clear () ; } self . undo_log . num_open_snapshots -= 1 ; } }
};
}
