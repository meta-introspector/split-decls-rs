// Generated macro for HttpsConnector (struct)
macro_rules! Depcrate_clientHttpsConnector {
() => {
// Module: crate::client
// Provides: {"HttpsConnector"}
// Dependencies: {}
# [doc = " A Connector for the `https` scheme."] # [derive (Clone)] pub struct HttpsConnector < T > { force_https : bool , http : T , tls : TlsConnector , }
};
}
