// Generated macro for impl_18 (impl)
macro_rules! Depcrate_clientimpl_18 {
() => {
// Module: crate::client
// Provides: {"impl_18"}
// Dependencies: {}
impl < T > HttpsConnector < T > { # [doc = " Force the use of HTTPS when connecting."] # [doc = ""] # [doc = " If a URL is not `https` when connecting, an error is returned."] pub fn https_only (& mut self , enable : bool) { self . force_https = enable ; } # [doc = " With connector constructor"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This will panic if the underlying TLS context could not be created."] # [doc = ""] # [doc = " To handle that error yourself, you can use the `HttpsConnector::from`"] # [doc = " constructor after trying to make a `TlsConnector`."] pub fn new_with_connector (http : T) -> Self { native_tls :: TlsConnector :: new () . map_or_else (| e | { panic ! ("HttpsConnector::new_with_connector(<connector>) failure: {}" , e) } , | tls | HttpsConnector :: from ((http , tls . into ())) ,) } }
};
}
