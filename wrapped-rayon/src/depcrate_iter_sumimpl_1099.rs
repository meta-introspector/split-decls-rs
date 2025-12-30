// Generated macro for impl_1099 (impl)
macro_rules! Depcrate_iter_sumimpl_1099 {
() => {
// Module: crate::iter::sum
// Provides: {"impl_1099"}
// Dependencies: {}
impl < S , T > UnindexedConsumer < T > for SumConsumer < S > where S : Send + Sum < T > + Sum , { fn split_off_left (& self) -> Self { SumConsumer :: new () } fn to_reducer (& self) -> Self :: Reducer { SumConsumer :: new () } }
};
}
