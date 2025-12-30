// Generated macro for HttpsConnector (struct)
macro_rules! Depcrate_connectorHttpsConnector {
() => {
// Module: crate::connector
// Provides: {"HttpsConnector"}
// Dependencies: {}
# [doc = " A Connector for the `https` scheme."] # [derive (Clone)] pub struct HttpsConnector < T > { force_https : bool , http : T , tls_config : Arc < rustls :: ClientConfig > , server_name_resolver : Arc < dyn ResolveServerName + Sync + Send > , }
};
}
