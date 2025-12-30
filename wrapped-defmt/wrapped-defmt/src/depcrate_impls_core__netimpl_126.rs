// Generated macro for impl_126 (impl)
macro_rules! Depcrate_impls_core__netimpl_126 {
() => {
// Module: crate::impls::core_::net
// Provides: {"impl_126"}
// Dependencies: {}
impl Format for net :: SocketAddrV4 { fn format (& self , fmt : Formatter) { crate :: write ! (fmt , "{}:{}" , self . ip () , self . port ()) ; } }
};
}
