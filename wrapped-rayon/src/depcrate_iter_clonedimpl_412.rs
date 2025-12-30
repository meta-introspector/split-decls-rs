// Generated macro for impl_412 (impl)
macro_rules! Depcrate_iter_clonedimpl_412 {
() => {
// Module: crate::iter::cloned
// Provides: {"impl_412"}
// Dependencies: {}
impl < 'a , T , C > UnindexedConsumer < & 'a T > for ClonedConsumer < C > where C : UnindexedConsumer < T > , T : 'a + Clone , { fn split_off_left (& self) -> Self { ClonedConsumer :: new (self . base . split_off_left ()) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
