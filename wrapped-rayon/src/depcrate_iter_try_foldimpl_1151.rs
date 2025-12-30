// Generated macro for impl_1151 (impl)
macro_rules! Depcrate_iter_try_foldimpl_1151 {
() => {
// Module: crate::iter::try_fold
// Provides: {"impl_1151"}
// Dependencies: {}
impl < 'r , U , T , C , ID , F > UnindexedConsumer < T > for TryFoldConsumer < 'r , U , C , ID , F > where C : UnindexedConsumer < U > , F : Fn (U :: Output , T) -> U + Sync , ID : Fn () -> U :: Output + Sync , U : Try + Send , { fn split_off_left (& self) -> Self { TryFoldConsumer { base : self . base . split_off_left () , .. * self } } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
