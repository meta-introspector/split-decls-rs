// Generated macro for impl_830 (impl)
macro_rules! Depcrate_iter_intersperseimpl_830 {
() => {
// Module: crate::iter::intersperse
// Provides: {"impl_830"}
// Dependencies: {}
impl < C , T > UnindexedConsumer < T > for IntersperseConsumer < C , T > where C : UnindexedConsumer < T > , T : Clone + Send , { fn split_off_left (& self) -> Self { let left = IntersperseConsumer { base : self . base . split_off_left () , item : self . item . clone () , clone_first : self . clone_first . clone () , } ; self . clone_first . set (true) ; left } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
