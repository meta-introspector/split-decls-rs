// Generated macro for impl_36 (impl)
macro_rules! Depcrate_fragmentsimpl_36 {
() => {
// Module: crate::fragments
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'a > sval :: Value for BinaryBuf < 'a > { fn stream < 'sval , S : sval :: Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S) -> sval :: Result { sval :: BinarySlice :: new (self . as_slice ()) . stream (stream) } }
};
}
