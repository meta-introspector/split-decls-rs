// Generated macro for impl_863 (impl)
macro_rules! Depcrate_iter_mapimpl_863 {
() => {
// Module: crate::iter::map
// Provides: {"impl_863"}
// Dependencies: {}
impl < 'f , T , R , C , F > UnindexedConsumer < T > for MapConsumer < 'f , C , F > where C : UnindexedConsumer < F :: Output > , F : Fn (T) -> R + Sync , R : Send , { fn split_off_left (& self) -> Self { MapConsumer :: new (self . base . split_off_left () , self . map_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
