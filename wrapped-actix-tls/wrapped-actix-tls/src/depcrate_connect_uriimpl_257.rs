// Generated macro for impl_257 (impl)
macro_rules! Depcrate_connect_uriimpl_257 {
() => {
// Module: crate::connect::uri
// Provides: {"impl_257"}
// Dependencies: {}
impl Host for http_0_2 :: Uri { fn hostname (& self) -> & str { self . host () . unwrap_or ("") } fn port (& self) -> Option < u16 > { match self . port_u16 () { Some (port) => Some (port) , None => scheme_to_port (self . scheme_str ()) , } } }
};
}
