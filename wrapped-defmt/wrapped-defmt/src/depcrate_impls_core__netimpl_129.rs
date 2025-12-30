// Generated macro for impl_129 (impl)
macro_rules! Depcrate_impls_core__netimpl_129 {
() => {
// Module: crate::impls::core_::net
// Provides: {"impl_129"}
// Dependencies: {}
impl Format for net :: IpAddr { fn format (& self , fmt : Formatter) { match self { net :: IpAddr :: V4 (a) => crate :: write ! (fmt , "{}" , a) , net :: IpAddr :: V6 (a) => crate :: write ! (fmt , "{}" , a) , } } }
};
}
