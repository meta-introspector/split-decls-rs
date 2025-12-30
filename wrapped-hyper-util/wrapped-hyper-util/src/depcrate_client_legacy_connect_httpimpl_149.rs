// Generated macro for impl_149 (impl)
macro_rules! Depcrate_client_legacy_connect_httpimpl_149 {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"impl_149"}
// Dependencies: {}
impl ConnectError { fn new < E > (msg : & 'static str , cause : E) -> ConnectError where E : Into < Box < dyn StdError + Send + Sync > > , { ConnectError { msg , addr : None , cause : Some (cause . into ()) , } } fn dns < E > (cause : E) -> ConnectError where E : Into < Box < dyn StdError + Send + Sync > > , { ConnectError :: new ("dns error" , cause) } fn m < E > (msg : & 'static str) -> impl FnOnce (E) -> ConnectError where E : Into < Box < dyn StdError + Send + Sync > > , { move | cause | ConnectError :: new (msg , cause) } }
};
}
