// Generated macro for impl_211 (impl)
macro_rules! Depcrate_ordimpl_211 {
() => {
// Module: crate::ord
// Provides: {"impl_211"}
// Dependencies: {}
# [doc = " Provide a no-op implementation for PhantomData"] impl < T > DerOrd for PhantomData < T > { fn der_cmp (& self , _other : & Self) -> Result < Ordering > { Ok (Ordering :: Equal) } }
};
}
