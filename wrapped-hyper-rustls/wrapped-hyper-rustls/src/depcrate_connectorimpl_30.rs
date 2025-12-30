// Generated macro for impl_30 (impl)
macro_rules! Depcrate_connectorimpl_30 {
() => {
// Module: crate::connector
// Provides: {"impl_30"}
// Dependencies: {}
impl < T > fmt :: Debug for HttpsConnector < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("HttpsConnector") . field ("force_https" , & self . force_https) . finish () } }
};
}
