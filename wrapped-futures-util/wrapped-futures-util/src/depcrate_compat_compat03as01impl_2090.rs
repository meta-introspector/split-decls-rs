// Generated macro for impl_2090 (impl)
macro_rules! Depcrate_compat_compat03as01impl_2090 {
() => {
// Module: crate::compat::compat03as01
// Provides: {"impl_2090"}
// Dependencies: {}
impl < T > Compat < T > { # [doc = " Creates a new [`Compat`]."] # [doc = ""] # [doc = " For types which implement appropriate futures `0.3`"] # [doc = " traits, the result will be a type which implements"] # [doc = " the corresponding futures 0.1 type."] pub fn new (inner : T) -> Self { Self { inner } } # [doc = " Get a reference to 0.3 Future, Stream, AsyncRead, or AsyncWrite object"] # [doc = " contained within."] pub fn get_ref (& self) -> & T { & self . inner } # [doc = " Get a mutable reference to 0.3 Future, Stream, AsyncRead, or AsyncWrite object"] # [doc = " contained within."] pub fn get_mut (& mut self) -> & mut T { & mut self . inner } # [doc = " Returns the inner item."] pub fn into_inner (self) -> T { self . inner } }
};
}
