// Generated macro for impl_897 (impl)
macro_rules! Depcrate_iter_map_withimpl_897 {
() => {
// Module: crate::iter::map_with
// Provides: {"impl_897"}
// Dependencies: {}
impl < 'f , T , INIT , U , R , C , F > UnindexedConsumer < T > for MapInitConsumer < 'f , C , INIT , F > where C : UnindexedConsumer < R > , INIT : Fn () -> U + Sync , F : Fn (& mut U , T) -> R + Sync , R : Send , { fn split_off_left (& self) -> Self { MapInitConsumer :: new (self . base . split_off_left () , self . init , self . map_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
