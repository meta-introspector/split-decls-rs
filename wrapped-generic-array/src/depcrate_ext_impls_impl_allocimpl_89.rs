// Generated macro for impl_89 (impl)
macro_rules! Depcrate_ext_impls_impl_allocimpl_89 {
() => {
// Module: crate::ext_impls::impl_alloc
// Provides: {"impl_89"}
// Dependencies: {}
impl < T , N : ArrayLength > FromIterator < T > for Box < GenericArray < T , N > > { # [doc = " Create a `Box<GenericArray>` from an iterator."] # [doc = ""] # [doc = " Will panic if the number of elements is not exactly the array length."] # [doc = ""] # [doc = " See [`GenericArray::try_boxed_from_iter]` for a fallible alternative."] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { match GenericArray :: try_boxed_from_iter (iter) { Ok (res) => res , Err (_) => crate :: from_iter_length_fail (N :: USIZE) , } } }
};
}
