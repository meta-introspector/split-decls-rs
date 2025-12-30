// Generated macro for impl_25 (impl)
macro_rules! Depcrate_fragmentsimpl_25 {
() => {
// Module: crate::fragments
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a > sval :: Value for TextBuf < 'a > { fn stream < 'sval , S : sval :: Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S) -> sval :: Result { self . as_str () . stream (stream) } }
};
}
