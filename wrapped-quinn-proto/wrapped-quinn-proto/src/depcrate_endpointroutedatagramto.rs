// Generated macro for RouteDatagramTo (enum)
macro_rules! Depcrate_endpointRouteDatagramTo {
() => {
// Module: crate::endpoint
// Provides: {"RouteDatagramTo"}
// Dependencies: {}
# [doc = " Part of protocol state incoming datagrams can be routed to"] # [derive (Copy , Clone , Debug)] enum RouteDatagramTo { Incoming (usize) , Connection (ConnectionHandle) , }
};
}
