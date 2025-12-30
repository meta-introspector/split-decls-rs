// Generated macro for impl_1134 (impl)
macro_rules! Depcrate_iter_take_any_whileimpl_1134 {
() => {
// Module: crate::iter::take_any_while
// Provides: {"impl_1134"}
// Dependencies: {}
impl < 'p , T , C , P > UnindexedConsumer < T > for TakeAnyWhileConsumer < 'p , C , P > where C : UnindexedConsumer < T > , P : Fn (& T) -> bool + Sync , { fn split_off_left (& self) -> Self { TakeAnyWhileConsumer { base : self . base . split_off_left () , .. * self } } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
