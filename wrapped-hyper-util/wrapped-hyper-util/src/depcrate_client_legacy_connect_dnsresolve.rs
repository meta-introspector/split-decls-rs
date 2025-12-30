// Generated macro for resolve (function)
macro_rules! Depcrate_client_legacy_connect_dnsresolve {
() => {
// Module: crate::client::legacy::connect::dns
// Provides: {"resolve"}
// Dependencies: {}
pub (super) async fn resolve < R > (resolver : & mut R , name : Name) -> Result < R :: Addrs , R :: Error > where R : Resolve , { crate :: common :: future :: poll_fn (| cx | resolver . poll_ready (cx)) . await ? ; resolver . resolve (name) . await }
};
}
