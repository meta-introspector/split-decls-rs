// Generated macro for MtuDiscovery (struct)
macro_rules! Depcrate_connection_mtudMtuDiscovery {
() => {
// Module: crate::connection::mtud
// Provides: {"MtuDiscovery"}
// Dependencies: {}
# [doc = " Implements Datagram Packetization Layer Path Maximum Transmission Unit Discovery"] # [doc = ""] # [doc = " See [`MtuDiscoveryConfig`] for details"] # [derive (Clone)] pub (crate) struct MtuDiscovery { # [doc = " Detected MTU for the path"] current_mtu : u16 , # [doc = " The state of the MTU discovery, if enabled"] state : Option < EnabledMtuDiscovery > , # [doc = " The state of the black hole detector"] black_hole_detector : BlackHoleDetector , }
};
}
