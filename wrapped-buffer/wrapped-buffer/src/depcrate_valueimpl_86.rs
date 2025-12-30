// Generated macro for impl_86 (impl)
macro_rules! Depcrate_valueimpl_86 {
() => {
// Module: crate::value
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'a > sval :: Value for ValueSlice < 'a > { fn stream < 'sval , S : sval :: Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S) -> sval :: Result { self . stream_ref (stream) } }
};
}
