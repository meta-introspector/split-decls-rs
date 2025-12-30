// Generated macro for impl_342 (impl)
macro_rules! Depcrate_rsliceimpl_342 {
() => {
// Module: crate::rslice
// Provides: {"impl_342"}
// Dependencies: {}
impl < 'a > From < & 'a [u8] > for rustls_slice_bytes < 'a > { fn from (s : & [u8]) -> Self { rustls_slice_bytes { data : s . as_ptr () , len : s . len () , phantom : PhantomData , } } }
};
}
