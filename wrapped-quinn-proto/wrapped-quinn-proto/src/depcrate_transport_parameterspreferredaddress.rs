// Generated macro for PreferredAddress (struct)
macro_rules! Depcrate_transport_parametersPreferredAddress {
() => {
// Module: crate::transport_parameters
// Provides: {"PreferredAddress"}
// Dependencies: {}
# [doc = " A server's preferred address"] # [doc = ""] # [doc = " This is communicated as a transport parameter during TLS session establishment."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub (crate) struct PreferredAddress { pub (crate) address_v4 : Option < SocketAddrV4 > , pub (crate) address_v6 : Option < SocketAddrV6 > , pub (crate) connection_id : ConnectionId , pub (crate) stateless_reset_token : ResetToken , }
};
}
