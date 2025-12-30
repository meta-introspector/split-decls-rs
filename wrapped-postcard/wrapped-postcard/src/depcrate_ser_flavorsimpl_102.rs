// Generated macro for impl_102 (impl)
macro_rules! Depcrate_ser_flavorsimpl_102 {
() => {
// Module: crate::ser::flavors
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'a > Slice < 'a > { # [doc = " Create a new `Slice` flavor from a given backing buffer"] pub fn new (buf : & 'a mut [u8]) -> Self { let ptr = buf . as_mut_ptr_range () ; Slice { start : ptr . start , cursor : ptr . start , end : ptr . end , _pl : PhantomData , } } }
};
}
