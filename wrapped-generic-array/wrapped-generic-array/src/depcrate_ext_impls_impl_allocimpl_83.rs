// Generated macro for impl_83 (impl)
macro_rules! Depcrate_ext_impls_impl_allocimpl_83 {
() => {
// Module: crate::ext_impls::impl_alloc
// Provides: {"impl_83"}
// Dependencies: {}
impl < T , N : ArrayLength > TryFrom < Vec < T > > for GenericArray < T , N > { type Error = crate :: LengthError ; fn try_from (v : Vec < T >) -> Result < Self , Self :: Error > { if v . len () != N :: USIZE { return Err (crate :: LengthError) ; } unsafe { let mut destination = core :: mem :: MaybeUninit :: < GenericArray < T , N > > :: uninit () ; let mut builder = IntrusiveArrayBuilder :: new_alt (& mut destination) ; builder . extend (v . into_iter ()) ; Ok (builder . finish_and_assume_init ()) } } }
};
}
