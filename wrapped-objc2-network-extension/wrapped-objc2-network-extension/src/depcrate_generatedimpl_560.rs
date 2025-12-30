// Generated macro for impl_560 (impl)
macro_rules! Depcrate_generatedimpl_560 {
() => {
// Module: crate::generated
// Provides: {"impl_560"}
// Dependencies: {}
impl NEPacket { extern_methods ! (# [cfg (feature = "libc")] # [doc = " Initializes a new NEPacket object with data and protocol family."] # [doc = ""] # [doc = " Parameter `data`: The content of the packet."] # [doc = ""] # [doc = " Parameter `protocolFamily`: The protocol family of the packet (such as AF_INET or AF_INET6)."] # [unsafe (method (initWithData : protocolFamily :))] # [unsafe (method_family = init)] pub unsafe fn initWithData_protocolFamily (this : Allocated < Self >, data : & NSData , protocol_family : libc :: sa_family_t ,) -> Retained < Self >; # [doc = " The data content of the packet."] # [unsafe (method (data))] # [unsafe (method_family = none)] pub unsafe fn data (& self) -> Retained < NSData >; # [cfg (feature = "libc")] # [doc = " The protocol family of the packet (such as AF_INET or AF_INET6)."] # [unsafe (method (protocolFamily))] # [unsafe (method_family = none)] pub unsafe fn protocolFamily (& self) -> libc :: sa_family_t ; # [doc = " The direction of the packet."] # [unsafe (method (direction))] # [unsafe (method_family = none)] pub unsafe fn direction (& self) -> NETrafficDirection ; # [doc = " Metadata about the source application and flow for this packet."] # [doc = " This property will only be non-nil when the routing method for the NEPacketTunnelProvider"] # [doc = " is NETunnelProviderRoutingMethodSourceApplication."] # [unsafe (method (metadata))] # [unsafe (method_family = none)] pub unsafe fn metadata (& self) -> Option < Retained < NEFlowMetaData >>;) ; }
};
}
