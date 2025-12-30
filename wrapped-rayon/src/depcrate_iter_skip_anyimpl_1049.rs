// Generated macro for impl_1049 (impl)
macro_rules! Depcrate_iter_skip_anyimpl_1049 {
() => {
// Module: crate::iter::skip_any
// Provides: {"impl_1049"}
// Dependencies: {}
impl < 'f , T , C > UnindexedConsumer < T > for SkipAnyConsumer < 'f , C > where C : UnindexedConsumer < T > , T : Send , { fn split_off_left (& self) -> Self { SkipAnyConsumer { base : self . base . split_off_left () , .. * self } } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
