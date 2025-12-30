// Generated macro for impl_594 (impl)
macro_rules! Depcrate_iter_filter_mapimpl_594 {
() => {
// Module: crate::iter::filter_map
// Provides: {"impl_594"}
// Dependencies: {}
impl < 'p , T , U , C , P > UnindexedConsumer < T > for FilterMapConsumer < 'p , C , P > where C : UnindexedConsumer < U > , P : Fn (T) -> Option < U > + Sync + 'p , { fn split_off_left (& self) -> Self { FilterMapConsumer :: new (self . base . split_off_left () , self . filter_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
