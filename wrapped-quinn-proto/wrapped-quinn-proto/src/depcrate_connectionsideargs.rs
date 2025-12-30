// Generated macro for SideArgs (enum)
macro_rules! Depcrate_connectionSideArgs {
() => {
// Module: crate::connection
// Provides: {"SideArgs"}
// Dependencies: {}
# [doc = " Parameters to `Connection::new` specific to it being client-side or server-side"] pub (crate) enum SideArgs { Client { token_store : Arc < dyn TokenStore > , server_name : String , } , Server { server_config : Arc < ServerConfig > , pref_addr_cid : Option < ConnectionId > , path_validated : bool , } , }
};
}
