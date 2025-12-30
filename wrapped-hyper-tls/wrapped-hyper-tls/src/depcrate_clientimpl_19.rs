// Generated macro for impl_19 (impl)
macro_rules! Depcrate_clientimpl_19 {
() => {
// Module: crate::client
// Provides: {"impl_19"}
// Dependencies: {}
impl < T > From < (T , TlsConnector) > for HttpsConnector < T > { fn from (args : (T , TlsConnector)) -> HttpsConnector < T > { HttpsConnector { force_https : false , http : args . 0 , tls : args . 1 , } } }
};
}
