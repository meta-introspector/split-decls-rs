// Generated macro for impl_16 (impl)
macro_rules! Depcrate_clientimpl_16 {
() => {
// Module: crate::client
// Provides: {"impl_16"}
// Dependencies: {}
impl HttpsConnector < HttpConnector > { # [doc = " Construct a new `HttpsConnector`."] # [doc = ""] # [doc = " This uses hyper's default `HttpConnector`, and default `TlsConnector`."] # [doc = " If you wish to use something besides the defaults, use `From::from`."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " By default this connector will use plain HTTP if the URL provided uses"] # [doc = " the HTTP scheme (eg: <http://example.com/>)."] # [doc = ""] # [doc = " If you would like to force the use of HTTPS then call `https_only(true)`"] # [doc = " on the returned connector."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This will panic if the underlying TLS context could not be created."] # [doc = ""] # [doc = " To handle that error yourself, you can use the `HttpsConnector::from`"] # [doc = " constructor after trying to make a `TlsConnector`."] # [must_use] pub fn new () -> Self { native_tls :: TlsConnector :: new () . map_or_else (| e | panic ! ("HttpsConnector::new() failure: {}" , e) , | tls | HttpsConnector :: new_ (tls . into ()) ,) } fn new_ (tls : TlsConnector) -> Self { let mut http = HttpConnector :: new () ; http . enforce_http (false) ; HttpsConnector :: from ((http , tls)) } }
};
}
