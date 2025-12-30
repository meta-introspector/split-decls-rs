// Generated macro for impl_1119 (impl)
macro_rules! Depcrate_iter_take_anyimpl_1119 {
() => {
// Module: crate::iter::take_any
// Provides: {"impl_1119"}
// Dependencies: {}
impl < 'f , T , C > UnindexedConsumer < T > for TakeAnyConsumer < 'f , C > where C : UnindexedConsumer < T > , T : Send , { fn split_off_left (& self) -> Self { TakeAnyConsumer { base : self . base . split_off_left () , .. * self } } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
