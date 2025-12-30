// Generated macro for impl_1160 (impl)
macro_rules! Depcrate_iter_try_foldimpl_1160 {
() => {
// Module: crate::iter::try_fold
// Provides: {"impl_1160"}
// Dependencies: {}
impl < 'r , U , T , C , F > UnindexedConsumer < T > for TryFoldWithConsumer < 'r , C , U , F > where C : UnindexedConsumer < U > , F : Fn (U :: Output , T) -> U + Sync , U : Try < Output : Clone + Send > + Send , { fn split_off_left (& self) -> Self { TryFoldWithConsumer { base : self . base . split_off_left () , item : self . item . clone () , .. * self } } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
