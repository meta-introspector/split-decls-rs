// Generated macro for impl_556 (impl)
macro_rules! Depcrate_snapshot_mapimpl_556 {
() => {
// Module: crate::snapshot_map
// Provides: {"impl_556"}
// Dependencies: {}
impl < K , V , M , L > Rollback < UndoLog < K , V > > for SnapshotMap < K , V , M , L > where K : Eq + Hash , M : Rollback < UndoLog < K , V > > , { fn reverse (& mut self , undo : UndoLog < K , V >) { self . map . reverse (undo) } }
};
}
