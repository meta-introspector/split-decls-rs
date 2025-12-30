// Generated macro for impl_996 (impl)
macro_rules! Depcrate_iter_reduceimpl_996 {
() => {
// Module: crate::iter::reduce
// Provides: {"impl_996"}
// Dependencies: {}
impl < 'r , R , ID , T > UnindexedConsumer < T > for ReduceConsumer < 'r , R , ID > where R : Fn (T , T) -> T + Sync , ID : Fn () -> T + Sync , T : Send , { fn split_off_left (& self) -> Self { * self } fn to_reducer (& self) -> Self :: Reducer { * self } }
};
}
