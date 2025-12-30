// Generated macro for impl_732 (impl)
macro_rules! Depcrate_generatedimpl_732 {
() => {
// Module: crate::generated
// Provides: {"impl_732"}
// Dependencies: {}
impl NWHostEndpoint { extern_methods ! (# [doc = " Parameter `hostname`: A string representation of the hostname or address, such as www.apple.com or 10.0.0.1."] # [doc = ""] # [doc = " Parameter `port`: A string containing the port on the host, such as 80."] # [doc = ""] # [doc = " Returns: An initialized NWHostEndpoint object."] # [deprecated = "Use `nw_endpoint_create_host` in Network framework instead, see deprecation notice in <NetworkExtension/NWHostEndpoint.h>"] # [unsafe (method (endpointWithHostname : port :))] # [unsafe (method_family = none)] pub unsafe fn endpointWithHostname_port (hostname : & NSString , port : & NSString ,) -> Retained < Self >; # [doc = " The endpoint's hostname."] # [deprecated = "Use `nw_endpoint_get_hostname` in Network framework instead, see deprecation notice in <NetworkExtension/NWHostEndpoint.h>"] # [unsafe (method (hostname))] # [unsafe (method_family = none)] pub unsafe fn hostname (& self) -> Retained < NSString >; # [doc = " The endpoint's port."] # [deprecated = "Use `nw_endpoint_get_port` in Network framework instead, see deprecation notice in <NetworkExtension/NWHostEndpoint.h>"] # [unsafe (method (port))] # [unsafe (method_family = none)] pub unsafe fn port (& self) -> Retained < NSString >;) ; }
};
}
