// Generated macro for impl_946 (impl)
macro_rules! Depcrate_iter_panic_fuseimpl_946 {
() => {
// Module: crate::iter::panic_fuse
// Provides: {"impl_946"}
// Dependencies: {}
impl < 'a , T , C > Reducer < T > for PanicFuseReducer < 'a , C > where C : Reducer < T > , { fn reduce (self , left : T , right : T) -> T { self . base . reduce (left , right) } }
};
}
