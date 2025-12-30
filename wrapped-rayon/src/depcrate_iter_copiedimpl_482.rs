// Generated macro for impl_482 (impl)
macro_rules! Depcrate_iter_copiedimpl_482 {
() => {
// Module: crate::iter::copied
// Provides: {"impl_482"}
// Dependencies: {}
impl < 'a , T , C > UnindexedConsumer < & 'a T > for CopiedConsumer < C > where C : UnindexedConsumer < T > , T : 'a + Copy , { fn split_off_left (& self) -> Self { CopiedConsumer :: new (self . base . split_off_left ()) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
