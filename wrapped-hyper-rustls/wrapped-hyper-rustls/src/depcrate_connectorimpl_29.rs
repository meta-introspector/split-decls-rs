// Generated macro for impl_29 (impl)
macro_rules! Depcrate_connectorimpl_29 {
() => {
// Module: crate::connector
// Provides: {"impl_29"}
// Dependencies: {}
impl < H , C > From < (H , C) > for HttpsConnector < H > where C : Into < Arc < rustls :: ClientConfig > > , { fn from ((http , cfg) : (H , C)) -> Self { Self { force_https : false , http , tls_config : cfg . into () , server_name_resolver : Arc :: new (DefaultServerNameResolver :: default ()) , } } }
};
}
