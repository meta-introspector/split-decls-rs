// Generated macro for impl_73 (impl)
macro_rules! Depcrateimpl_73 {
() => {
// Module: crate
// Provides: {"impl_73"}
// Dependencies: {}
impl < T > EventListener < T > { # [inline] fn listener (& self) -> & InnerListener < T , Arc < Inner < T > > > { & self . listener } # [inline] fn listener_mut (& mut self) -> Pin < & mut InnerListener < T , Arc < Inner < T > > > > { self . listener . as_mut () } }
};
}
