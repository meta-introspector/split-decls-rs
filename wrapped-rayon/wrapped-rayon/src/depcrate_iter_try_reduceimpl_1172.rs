// Generated macro for impl_1172 (impl)
macro_rules! Depcrate_iter_try_reduceimpl_1172 {
() => {
// Module: crate::iter::try_reduce
// Provides: {"impl_1172"}
// Dependencies: {}
impl < 'r , R , ID , T > UnindexedConsumer < T > for TryReduceConsumer < 'r , R , ID > where R : Fn (T :: Output , T :: Output) -> T + Sync , ID : Fn () -> T :: Output + Sync , T : Try + Send , { fn split_off_left (& self) -> Self { * self } fn to_reducer (& self) -> Self :: Reducer { * self } }
};
}
