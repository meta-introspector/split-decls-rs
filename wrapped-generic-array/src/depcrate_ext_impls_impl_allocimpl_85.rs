// Generated macro for impl_85 (impl)
macro_rules! Depcrate_ext_impls_impl_allocimpl_85 {
() => {
// Module: crate::ext_impls::impl_alloc
// Provides: {"impl_85"}
// Dependencies: {}
impl < T , N : ArrayLength > TryFrom < Box < [T] > > for GenericArray < T , N > { type Error = crate :: LengthError ; # [inline] fn try_from (value : Box < [T] >) -> Result < Self , Self :: Error > { Vec :: from (value) . try_into () } }
};
}
