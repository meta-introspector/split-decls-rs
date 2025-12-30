// Generated macro for impl_55 (impl)
macro_rules! Depcrate_implsimpl_55 {
() => {
// Module: crate::impls
// Provides: {"impl_55"}
// Dependencies: {}
impl < T , const N : usize > From < GenericArray < T , ConstArrayLength < N > > > for [T ; N] where Const < N > : IntoArrayLength , { # [inline (always)] fn from (value : GenericArray < T , ConstArrayLength < N > >) -> Self { value . into_array () } }
};
}
