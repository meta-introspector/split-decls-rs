// Generated macro for impl_1277 (impl)
macro_rules! Depcrate_iter_while_someimpl_1277 {
() => {
// Module: crate::iter::while_some
// Provides: {"impl_1277"}
// Dependencies: {}
impl < 'f , T , C > UnindexedConsumer < Option < T > > for WhileSomeConsumer < 'f , C > where C : UnindexedConsumer < T > , T : Send , { fn split_off_left (& self) -> Self { WhileSomeConsumer { base : self . base . split_off_left () , .. * self } } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
