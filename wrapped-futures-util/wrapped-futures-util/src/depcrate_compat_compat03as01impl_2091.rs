// Generated macro for impl_2091 (impl)
macro_rules! Depcrate_compat_compat03as01impl_2091 {
() => {
// Module: crate::compat::compat03as01
// Provides: {"impl_2091"}
// Dependencies: {}
# [cfg (feature = "sink")] impl < T , Item > CompatSink < T , Item > { # [doc = " Creates a new [`CompatSink`]."] pub fn new (inner : T) -> Self { Self { inner , _phantom : PhantomData } } # [doc = " Get a reference to 0.3 Sink contained within."] pub fn get_ref (& self) -> & T { & self . inner } # [doc = " Get a mutable reference to 0.3 Sink contained within."] pub fn get_mut (& mut self) -> & mut T { & mut self . inner } # [doc = " Returns the inner item."] pub fn into_inner (self) -> T { self . inner } }
};
}
