// Generated macro for impl_222 (impl)
macro_rules! Depcrate_registryimpl_222 {
() => {
// Module: crate::registry
// Provides: {"impl_222"}
// Dependencies: {}
impl HttpServerHandle { pub fn index_url (& self) -> Url { Url :: parse (& format ! ("sparse+http://{}/index/" , self . addr)) . unwrap () } pub fn api_url (& self) -> Url { Url :: parse (& format ! ("http://{}/" , self . addr)) . unwrap () } pub fn dl_url (& self) -> Url { Url :: parse (& format ! ("http://{}/dl" , self . addr)) . unwrap () } fn stop (& self) { if let Ok (mut stream) = TcpStream :: connect (self . addr) { let _ = stream . write_all (b"stop") ; let _ = stream . flush () ; } } }
};
}
