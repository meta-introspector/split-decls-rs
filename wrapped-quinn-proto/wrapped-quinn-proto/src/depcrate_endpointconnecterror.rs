// Generated macro for ConnectError (enum)
macro_rules! Depcrate_endpointConnectError {
() => {
// Module: crate::endpoint
// Provides: {"ConnectError"}
// Dependencies: {}
# [doc = " Errors in the parameters being used to create a new connection"] # [doc = ""] # [doc = " These arise before any I/O has been performed."] # [derive (Debug , Error , Clone , PartialEq , Eq)] pub enum ConnectError { # [doc = " The endpoint can no longer create new connections"] # [doc = ""] # [doc = " Indicates that a necessary component of the endpoint has been dropped or otherwise disabled."] # [error ("endpoint stopping")] EndpointStopping , # [doc = " The connection could not be created because not enough of the CID space is available"] # [doc = ""] # [doc = " Try using longer connection IDs"] # [error ("CIDs exhausted")] CidsExhausted , # [doc = " The given server name was malformed"] # [error ("invalid server name: {0}")] InvalidServerName (String) , # [doc = " The remote [`SocketAddr`] supplied was malformed"] # [doc = ""] # [doc = " Examples include attempting to connect to port 0, or using an inappropriate address family."] # [error ("invalid remote address: {0}")] InvalidRemoteAddress (SocketAddr) , # [doc = " No default client configuration was set up"] # [doc = ""] # [doc = " Use `Endpoint::connect_with` to specify a client configuration."] # [error ("no default client config")] NoDefaultClientConfig , # [doc = " The local endpoint does not support the QUIC version specified in the client configuration"] # [error ("unsupported QUIC version")] UnsupportedVersion , }
};
}
