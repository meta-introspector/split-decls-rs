// Generated macro for impl_942 (impl)
macro_rules! Depcrate_iter_panic_fuseimpl_942 {
() => {
// Module: crate::iter::panic_fuse
// Provides: {"impl_942"}
// Dependencies: {}
impl < 'a , T , C > UnindexedConsumer < T > for PanicFuseConsumer < 'a , C > where C : UnindexedConsumer < T > , { fn split_off_left (& self) -> Self { PanicFuseConsumer { base : self . base . split_off_left () , fuse : self . fuse . clone () , } } fn to_reducer (& self) -> Self :: Reducer { PanicFuseReducer { base : self . base . to_reducer () , _fuse : self . fuse . clone () , } } }
};
}
