// Generated macro for impl_605 (impl)
macro_rules! Depcrate_iter_findimpl_605 {
() => {
// Module: crate::iter::find
// Provides: {"impl_605"}
// Dependencies: {}
impl < 'p , T , P : 'p > UnindexedConsumer < T > for FindConsumer < 'p , P > where T : Send , P : Fn (& T) -> bool + Sync , { fn split_off_left (& self) -> Self { FindConsumer :: new (self . find_op , self . found) } fn to_reducer (& self) -> Self :: Reducer { FindReducer } }
};
}
