// Generated macro for impl_1527 (impl)
macro_rules! Depcrate_stream_iterimpl_1527 {
() => {
// Module: crate::stream::iter
// Provides: {"impl_1527"}
// Dependencies: {}
impl < I > Iter < I > { # [doc = " Acquires a reference to the underlying iterator that this stream is pulling from."] pub fn get_ref (& self) -> & I { & self . iter } # [doc = " Acquires a mutable reference to the underlying iterator that this stream is pulling from."] pub fn get_mut (& mut self) -> & mut I { & mut self . iter } # [doc = " Consumes this stream, returning the underlying iterator."] pub fn into_inner (self) -> I { self . iter } }
};
}
