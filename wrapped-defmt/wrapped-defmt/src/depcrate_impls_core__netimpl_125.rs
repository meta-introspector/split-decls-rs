// Generated macro for impl_125 (impl)
macro_rules! Depcrate_impls_core__netimpl_125 {
() => {
// Module: crate::impls::core_::net
// Provides: {"impl_125"}
// Dependencies: {}
impl Format for net :: Ipv4Addr { fn format (& self , fmt : Formatter) { let [a , b , c , d] = self . octets () ; crate :: write ! (fmt , "{}.{}.{}.{}" , a , b , c , d) ; } }
};
}
