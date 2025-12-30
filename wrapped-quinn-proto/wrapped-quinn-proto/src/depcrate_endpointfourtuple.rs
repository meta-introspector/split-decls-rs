// Generated macro for FourTuple (struct)
macro_rules! Depcrate_endpointFourTuple {
() => {
// Module: crate::endpoint
// Provides: {"FourTuple"}
// Dependencies: {}
# [doc = " Identifies a connection by the combination of remote and local addresses"] # [doc = ""] # [doc = " Including the local ensures good behavior when the host has multiple IP addresses on the same"] # [doc = " subnet and zero-length connection IDs are in use."] # [derive (Hash , Eq , PartialEq , Debug , Copy , Clone)] struct FourTuple { remote : SocketAddr , local_ip : Option < IpAddr > , }
};
}
