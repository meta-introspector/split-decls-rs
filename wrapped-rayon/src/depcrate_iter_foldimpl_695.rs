// Generated macro for impl_695 (impl)
macro_rules! Depcrate_iter_foldimpl_695 {
() => {
// Module: crate::iter::fold
// Provides: {"impl_695"}
// Dependencies: {}
impl < 'r , U , T , C , ID , F > UnindexedConsumer < T > for FoldConsumer < 'r , C , ID , F > where C : UnindexedConsumer < U > , F : Fn (U , T) -> U + Sync , ID : Fn () -> U + Sync , U : Send , { fn split_off_left (& self) -> Self { FoldConsumer { base : self . base . split_off_left () , .. * self } } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
