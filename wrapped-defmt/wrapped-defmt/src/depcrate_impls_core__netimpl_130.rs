// Generated macro for impl_130 (impl)
macro_rules! Depcrate_impls_core__netimpl_130 {
() => {
// Module: crate::impls::core_::net
// Provides: {"impl_130"}
// Dependencies: {}
impl Format for net :: SocketAddr { fn format (& self , fmt : Formatter) { match self { net :: SocketAddr :: V4 (a) => crate :: write ! (fmt , "{}" , a) , net :: SocketAddr :: V6 (a) => crate :: write ! (fmt , "{}" , a) , } } }
};
}
