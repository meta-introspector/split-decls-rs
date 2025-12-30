// Generated macro for impl_37 (impl)
macro_rules! Depcrate_fragmentsimpl_37 {
() => {
// Module: crate::fragments
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'sval > sval_ref :: ValueRef < 'sval > for BinaryBuf < 'sval > { fn stream_ref < S : sval :: Stream < 'sval > + ? Sized > (& self , stream : & mut S) -> sval :: Result { match self . as_borrowed_slice () { Some (v) => stream . value (sval :: BinarySlice :: new (v)) , None => stream . value_computed (sval :: BinarySlice :: new (self . as_slice ())) , } } }
};
}
