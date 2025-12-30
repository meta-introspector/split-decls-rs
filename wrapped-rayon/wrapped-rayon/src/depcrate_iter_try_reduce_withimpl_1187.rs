// Generated macro for impl_1187 (impl)
macro_rules! Depcrate_iter_try_reduce_withimpl_1187 {
() => {
// Module: crate::iter::try_reduce_with
// Provides: {"impl_1187"}
// Dependencies: {}
impl < 'r , R , T > UnindexedConsumer < T > for TryReduceWithConsumer < 'r , R > where R : Fn (T :: Output , T :: Output) -> T + Sync , T : Try + Send , { fn split_off_left (& self) -> Self { * self } fn to_reducer (& self) -> Self :: Reducer { * self } }
};
}
