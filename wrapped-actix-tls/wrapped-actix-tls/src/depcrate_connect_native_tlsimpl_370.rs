// Generated macro for impl_370 (impl)
macro_rules! Depcrate_connect_native_tlsimpl_370 {
() => {
// Module: crate::connect::native_tls
// Provides: {"impl_370"}
// Dependencies: {}
impl TlsConnector { # [doc = " Constructs new connector service from a `native-tls` connector."] # [doc = ""] # [doc = " This type is it's own service factory, so it can be used in that setting, too."] pub fn new (connector : NativeTlsConnector) -> Self { Self { connector : AsyncNativeTlsConnector :: from (connector) , } } }
};
}
