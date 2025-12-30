// Generated macro for impl_1100 (impl)
macro_rules! Depcrate_iter_sumimpl_1100 {
() => {
// Module: crate::iter::sum
// Provides: {"impl_1100"}
// Dependencies: {}
impl < S > Reducer < S > for SumConsumer < S > where S : Send + Sum , { fn reduce (self , left : S , right : S) -> S { add (left , right) } }
};
}
