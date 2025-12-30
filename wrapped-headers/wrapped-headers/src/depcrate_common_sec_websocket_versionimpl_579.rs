// Generated macro for impl_579 (impl)
macro_rules! Depcrate_common_sec_websocket_versionimpl_579 {
() => {
// Module: crate::common::sec_websocket_version
// Provides: {"impl_579"}
// Dependencies: {}
impl Header for SecWebsocketVersion { fn name () -> & 'static HeaderName { & :: http :: header :: SEC_WEBSOCKET_VERSION } fn decode < 'i , I : Iterator < Item = & 'i HeaderValue > > (values : & mut I) -> Result < Self , Error > { values . next () . and_then (| value | { if value == "13" { Some (SecWebsocketVersion :: V13) } else { None } }) . ok_or_else (Error :: invalid) } fn encode < E : Extend < HeaderValue > > (& self , values : & mut E) { debug_assert_eq ! (self . 0 , 13) ; values . extend (:: std :: iter :: once (HeaderValue :: from_static ("13"))) ; } }
};
}
