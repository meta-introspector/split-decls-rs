// Generated macro for impl_128 (impl)
macro_rules! Depcrate_impls_core__netimpl_128 {
() => {
// Module: crate::impls::core_::net
// Provides: {"impl_128"}
// Dependencies: {}
impl Format for net :: SocketAddrV6 { fn format (& self , fmt : Formatter) { crate :: write ! (fmt , "[{}]:{}" , self . ip () , self . port ()) ; } }
};
}
