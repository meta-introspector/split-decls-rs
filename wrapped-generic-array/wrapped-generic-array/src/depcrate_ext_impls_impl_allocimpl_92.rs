// Generated macro for impl_92 (impl)
macro_rules! Depcrate_ext_impls_impl_allocimpl_92 {
() => {
// Module: crate::ext_impls::impl_alloc
// Provides: {"impl_92"}
// Dependencies: {}
unsafe impl < T , N : ArrayLength > GenericSequence < T > for Box < GenericArray < T , N > > { type Length = N ; type Sequence = Box < GenericArray < T , N > > ; fn generate < F > (mut f : F) -> Self :: Sequence where F : FnMut (usize) -> T , { unsafe { use core :: { alloc :: Layout , mem :: { size_of , MaybeUninit } , ptr , } ; let ptr : * mut GenericArray < MaybeUninit < T > , N > = if size_of :: < T > () == 0 { ptr :: NonNull :: dangling () . as_ptr () } else { alloc :: alloc :: alloc (Layout :: new :: < GenericArray < MaybeUninit < T > , N > > ()) . cast () } ; let mut builder = IntrusiveArrayBuilder :: new (& mut * ptr) ; { let (builder_iter , position) = builder . iter_position () ; builder_iter . enumerate () . for_each (| (i , dst) | { dst . write (f (i)) ; * position += 1 ; }) ; } builder . finish () ; Box :: from_raw (ptr . cast ()) } } }
};
}
