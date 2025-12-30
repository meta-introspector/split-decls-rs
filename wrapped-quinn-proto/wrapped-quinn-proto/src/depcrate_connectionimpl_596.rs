// Generated macro for impl_596 (impl)
macro_rules! Depcrate_connectionimpl_596 {
() => {
// Module: crate::connection
// Provides: {"impl_596"}
// Dependencies: {}
impl From < SideArgs > for ConnectionSide { fn from (side : SideArgs) -> Self { match side { SideArgs :: Client { token_store , server_name , } => Self :: Client { token : token_store . take (& server_name) . unwrap_or_default () , token_store , server_name , } , SideArgs :: Server { server_config , pref_addr_cid : _ , path_validated : _ , } => Self :: Server { server_config } , } } }
};
}
