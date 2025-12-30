// Generated macro for impl_37 (impl)
macro_rules! Depcrate_de_flavorsimpl_37 {
() => {
// Module: crate::de::flavors
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'de > Slice < 'de > { # [doc = " Create a new [Slice] from the given buffer"] pub fn new (sli : & 'de [u8]) -> Self { let range = sli . as_ptr_range () ; Self { cursor : range . start , end : range . end , _pl : PhantomData , } } }
};
}
