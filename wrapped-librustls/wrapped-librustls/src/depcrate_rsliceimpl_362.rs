// Generated macro for impl_362 (impl)
macro_rules! Depcrate_rsliceimpl_362 {
() => {
// Module: crate::rslice
// Provides: {"impl_362"}
// Dependencies: {}
impl < 'a > From < & 'a [u16] > for rustls_slice_u16 < 'a > { fn from (s : & [u16]) -> Self { rustls_slice_u16 { data : s . as_ptr () , len : s . len () , phantom : PhantomData , } } }
};
}
