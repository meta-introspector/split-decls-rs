// Generated macro for impl_56 (impl)
macro_rules! Depcrate_implsimpl_56 {
() => {
// Module: crate::impls
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'a , T , const N : usize > From < & 'a [T ; N] > for & 'a GenericArray < T , ConstArrayLength < N > > where Const < N > : IntoArrayLength , { # [inline (always)] fn from (slice : & 'a [T ; N]) -> Self { unsafe { & * (slice . as_ptr () as * const GenericArray < T , ConstArrayLength < N > >) } } }
};
}
