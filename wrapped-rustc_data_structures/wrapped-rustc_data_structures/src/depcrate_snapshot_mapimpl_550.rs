// Generated macro for impl_550 (impl)
macro_rules! Depcrate_snapshot_mapimpl_550 {
() => {
// Module: crate::snapshot_map
// Provides: {"impl_550"}
// Dependencies: {}
impl < K , V , M , L > Default for SnapshotMap < K , V , M , L > where M : Default , L : Default , { fn default () -> Self { SnapshotMap { map : Default :: default () , undo_log : Default :: default () , _marker : PhantomData } } }
};
}
