// Generated macro for impl_21 (impl)
macro_rules! Depcrate_clientimpl_21 {
() => {
// Module: crate::client
// Provides: {"impl_21"}
// Dependencies: {}
impl < T > Service < Uri > for HttpsConnector < T > where T : Service < Uri > , T :: Response : Read + Write + Send + Unpin , T :: Future : Send + 'static , T :: Error : Into < BoxError > , { type Response = MaybeHttpsStream < T :: Response > ; type Error = BoxError ; type Future = HttpsConnecting < T :: Response > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { match self . http . poll_ready (cx) { Poll :: Ready (Ok (())) => Poll :: Ready (Ok (())) , Poll :: Ready (Err (e)) => Poll :: Ready (Err (e . into ())) , Poll :: Pending => Poll :: Pending , } } fn call (& mut self , dst : Uri) -> Self :: Future { let is_https = dst . scheme_str () == Some ("https") ; if ! is_https && self . force_https { return err (ForceHttpsButUriNotHttps . into ()) ; } let host = dst . host () . unwrap_or ("") . trim_matches (| c | c == '[' || c == ']') . to_owned () ; let connecting = self . http . call (dst) ; let tls_connector = self . tls . clone () ; let fut = async move { let tcp = connecting . await . map_err (Into :: into) ? ; let maybe = if is_https { let stream = TokioIo :: new (tcp) ; let tls = TokioIo :: new (tls_connector . connect (& host , stream) . await ?) ; MaybeHttpsStream :: Https (tls) } else { MaybeHttpsStream :: Http (tcp) } ; Ok (maybe) } ; HttpsConnecting (Box :: pin (fut)) } }
};
}
