// Generated macro for impl_1241 (impl)
macro_rules! Depcrate_iter_updateimpl_1241 {
() => {
// Module: crate::iter::update
// Provides: {"impl_1241"}
// Dependencies: {}
impl < 'f , T , C , F > UnindexedConsumer < T > for UpdateConsumer < 'f , C , F > where C : UnindexedConsumer < T > , F : Fn (& mut T) + Send + Sync , { fn split_off_left (& self) -> Self { UpdateConsumer :: new (self . base . split_off_left () , self . update_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
