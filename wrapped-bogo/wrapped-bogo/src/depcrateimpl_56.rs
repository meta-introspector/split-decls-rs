// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
impl server :: StoresServerSessions for ServerCacheWithResumptionDelay { fn put (& self , key : Vec < u8 > , value : Vec < u8 >) -> bool { let mut ssv = ServerSessionValue :: read_bytes (& value) . unwrap () ; match & mut ssv { ServerSessionValue :: Tls12 (tls12) => & mut tls12 . common , ServerSessionValue :: Tls13 (tls13) => & mut tls13 . common , _ => todo ! () , } . creation_time_sec -= self . delay as u64 ; self . storage . put (key , ssv . get_encoding ()) } fn get (& self , key : & [u8]) -> Option < Vec < u8 > > { self . storage . get (key) } fn take (& self , key : & [u8]) -> Option < Vec < u8 > > { self . storage . take (key) } fn can_cache (& self) -> bool { self . storage . can_cache () } }
};
}
