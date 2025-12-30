// Generated macro for impl_54 (impl)
macro_rules! Depcrate_implsimpl_54 {
() => {
// Module: crate::impls
// Provides: {"impl_54"}
// Dependencies: {}
impl < T , const N : usize > From < [T ; N] > for GenericArray < T , ConstArrayLength < N > > where Const < N > : IntoArrayLength , { # [inline (always)] fn from (value : [T ; N]) -> Self { GenericArray :: from_array (value) } }
};
}
