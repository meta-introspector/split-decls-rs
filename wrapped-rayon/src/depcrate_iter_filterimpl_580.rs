// Generated macro for impl_580 (impl)
macro_rules! Depcrate_iter_filterimpl_580 {
() => {
// Module: crate::iter::filter
// Provides: {"impl_580"}
// Dependencies: {}
impl < 'p , T , C , P : 'p > UnindexedConsumer < T > for FilterConsumer < 'p , C , P > where C : UnindexedConsumer < T > , P : Fn (& T) -> bool + Sync , { fn split_off_left (& self) -> Self { FilterConsumer :: new (self . base . split_off_left () , self . filter_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
