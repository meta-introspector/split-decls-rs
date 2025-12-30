// Generated macro for impl_884 (impl)
macro_rules! Depcrate_iter_map_withimpl_884 {
() => {
// Module: crate::iter::map_with
// Provides: {"impl_884"}
// Dependencies: {}
impl < 'f , T , U , R , C , F > UnindexedConsumer < T > for MapWithConsumer < 'f , C , U , F > where C : UnindexedConsumer < R > , U : Send + Clone , F : Fn (& mut U , T) -> R + Sync , R : Send , { fn split_off_left (& self) -> Self { MapWithConsumer :: new (self . base . split_off_left () , self . item . clone () , self . map_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
