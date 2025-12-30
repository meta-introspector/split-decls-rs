// Generated macro for impl_552 (impl)
macro_rules! Depcrate_snapshot_mapimpl_552 {
() => {
// Module: crate::snapshot_map
// Provides: {"impl_552"}
// Dependencies: {}
impl < K , V , M , L > SnapshotMap < K , V , M , L > { # [inline] pub fn with_log < L2 > (& mut self , undo_log : L2) -> SnapshotMap < K , V , & mut M , L2 > { SnapshotMap { map : & mut self . map , undo_log , _marker : PhantomData } } }
};
}
