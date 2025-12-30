// Generated macro for impl_1064 (impl)
macro_rules! Depcrate_iter_skip_any_whileimpl_1064 {
() => {
// Module: crate::iter::skip_any_while
// Provides: {"impl_1064"}
// Dependencies: {}
impl < 'p , T , C , P > UnindexedConsumer < T > for SkipAnyWhileConsumer < 'p , C , P > where C : UnindexedConsumer < T > , P : Fn (& T) -> bool + Sync , { fn split_off_left (& self) -> Self { SkipAnyWhileConsumer { base : self . base . split_off_left () , .. * self } } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
