// Generated macro for impl_72 (impl)
macro_rules! Depcrate_valueimpl_72 {
() => {
// Module: crate::value
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'a > sval :: Value for ValueBuf < 'a > { fn stream < 'sval , S : sval :: Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S) -> sval :: Result { self . stream_ref (stream) } }
};
}
