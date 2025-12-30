// Generated macro for impl_786 (impl)
macro_rules! Depcrate_iter_inspectimpl_786 {
() => {
// Module: crate::iter::inspect
// Provides: {"impl_786"}
// Dependencies: {}
impl < 'f , T , C , F > UnindexedConsumer < T > for InspectConsumer < 'f , C , F > where C : UnindexedConsumer < T > , F : Fn (& T) + Sync , { fn split_off_left (& self) -> Self { InspectConsumer :: new (self . base . split_off_left () , self . inspect_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
