// Generated macro for ConnectionSide (enum)
macro_rules! Depcrate_connectionConnectionSide {
() => {
// Module: crate::connection
// Provides: {"ConnectionSide"}
// Dependencies: {}
# [doc = " Fields of `Connection` specific to it being client-side or server-side"] enum ConnectionSide { Client { # [doc = " Sent in every outgoing Initial packet. Always empty after Initial keys are discarded"] token : Bytes , token_store : Arc < dyn TokenStore > , server_name : String , } , Server { server_config : Arc < ServerConfig > , } , }
};
}
