// Generated macro for impl_20 (impl)
macro_rules! Depcrate_clientimpl_20 {
() => {
// Module: crate::client
// Provides: {"impl_20"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for HttpsConnector < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("HttpsConnector") . field ("force_https" , & self . force_https) . field ("http" , & self . http) . finish_non_exhaustive () } }
};
}
