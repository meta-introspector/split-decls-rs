// Generated macro for transport_factory (function)
macro_rules! Depcrate_transporttransport_factory {
() => {
// Module: crate::transport
// Provides: {"transport_factory"}
// Dependencies: {}
extern "C" fn transport_factory (out : * mut * mut raw :: git_transport , owner : * mut raw :: git_remote , param : * mut c_void ,) -> c_int { struct Bomb < 'a > { remote : Option < Remote < 'a > > , } impl < 'a > Drop for Bomb < 'a > { fn drop (& mut self) { mem :: forget (self . remote . take ()) ; } } panic :: wrap (| | unsafe { let remote = Bomb { remote : Some (Binding :: from_raw (owner)) , } ; let data = & mut * (param as * mut TransportData) ; match (data . factory) (remote . remote . as_ref () . unwrap ()) { Ok (mut transport) => { * out = transport . raw ; transport . owned = false ; 0 } Err (e) => e . raw_code () as c_int , } }) . unwrap_or (- 1) }
};
}
