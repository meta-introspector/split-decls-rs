// Generated macro for impl_735 (impl)
macro_rules! Depcrate_iter_for_eachimpl_735 {
() => {
// Module: crate::iter::for_each
// Provides: {"impl_735"}
// Dependencies: {}
impl < 'f , F , T > UnindexedConsumer < T > for ForEachConsumer < 'f , F > where F : Fn (T) + Sync , { fn split_off_left (& self) -> Self { ForEachConsumer { op : self . op } } fn to_reducer (& self) -> NoopReducer { NoopReducer } }
};
}
