// Generated macro for NetworkInterface (struct)
macro_rules! Depcrate_executor_networkNetworkInterface {
() => {
// Module: crate::executor::network
// Provides: {"NetworkInterface"}
// Dependencies: {}
pub (crate) struct NetworkInterface < 'a > { pub (super) iface : smoltcp :: iface :: Interface , pub (super) sockets : SocketSet < 'a > , # [cfg (feature = "trace")] pub (super) device : smoltcp :: phy :: Tracer < NetworkDevice > , # [cfg (not (feature = "trace"))] pub (super) device : NetworkDevice , # [cfg (feature = "dhcpv4")] pub (super) dhcp_handle : SocketHandle , # [cfg (feature = "dns")] pub (super) dns_handle : Option < SocketHandle > , }
};
}
