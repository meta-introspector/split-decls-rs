// Generated macro for ConnectionState (struct)
macro_rules! Depcrate_client_gitConnectionState {
() => {
// Module: crate::client::git
// Provides: {"ConnectionState"}
// Dependencies: {}
# [doc = " Connection state shared between blocking and async connections."] pub (crate) struct ConnectionState { pub (in crate :: client) path : BString , pub (in crate :: client) virtual_host : Option < (String , Option < u16 >) > , pub (in crate :: client) desired_version : Protocol , custom_url : Option < BString > , pub (in crate :: client) mode : ConnectMode , }
};
}
