// Generated macro for Connected (struct)
macro_rules! Depcrate_client_legacy_connectConnected {
() => {
// Module: crate::client::legacy::connect
// Provides: {"Connected"}
// Dependencies: {}
# [doc = " Extra information about the connected transport."] # [doc = ""] # [doc = " This can be used to inform recipients about things like if ALPN"] # [doc = " was used, or if connected to an HTTP proxy."] # [derive (Debug)] pub struct Connected { pub (super) alpn : Alpn , pub (super) is_proxied : bool , pub (super) extra : Option < Extra > , pub (super) poisoned : PoisonPill , }
};
}
