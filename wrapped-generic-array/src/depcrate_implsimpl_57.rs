// Generated macro for impl_57 (impl)
macro_rules! Depcrate_implsimpl_57 {
() => {
// Module: crate::impls
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'a , T , const N : usize > From < & 'a mut [T ; N] > for & 'a mut GenericArray < T , ConstArrayLength < N > > where Const < N > : IntoArrayLength , { # [inline (always)] fn from (slice : & 'a mut [T ; N]) -> Self { unsafe { & mut * (slice . as_mut_ptr () as * mut GenericArray < T , ConstArrayLength < N > >) } } }
};
}
