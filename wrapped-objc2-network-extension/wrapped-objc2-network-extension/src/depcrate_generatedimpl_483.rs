// Generated macro for impl_483 (impl)
macro_rules! Depcrate_generatedimpl_483 {
() => {
// Module: crate::generated
// Provides: {"impl_483"}
// Dependencies: {}
impl NEEthernetTunnelNetworkSettings { extern_methods ! (# [doc = " This function initializes a newly-allocated NEEthernetTunnelNetworkSettings object with a given tunnel remote address and MAC address."] # [doc = ""] # [doc = " Parameter `address`: The address of the remote endpoint that is providing the tunnel service."] # [doc = ""] # [doc = " Parameter `ethernetAddress`: The ethernet address to be assigned to the tunnel interface. This string should be in the format \"xx:xx:xx:xx:xx:xx\", where each xx is a hexidecimal number between 0 and ff."] # [doc = ""] # [doc = " Parameter `mtu`: The MTU (Maxium Transmission Unit) in bytes to be assigned to the tunnel interface."] # [unsafe (method (initWithTunnelRemoteAddress : ethernetAddress : mtu :))] # [unsafe (method_family = init)] pub unsafe fn initWithTunnelRemoteAddress_ethernetAddress_mtu (this : Allocated < Self >, address : & NSString , ethernet_address : & NSString , mtu : NSInteger ,) -> Retained < Self >; # [doc = " An NSString object containing the ethernet address of the tunnel interface."] # [unsafe (method (ethernetAddress))] # [unsafe (method_family = none)] pub unsafe fn ethernetAddress (& self) -> Retained < NSString >;) ; }
};
}
