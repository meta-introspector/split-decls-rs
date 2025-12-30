// Generated macro for impl_213 (impl)
macro_rules! Depcrate_registryimpl_213 {
() => {
// Module: crate::registry
// Provides: {"impl_213"}
// Dependencies: {}
impl TestRegistry { pub fn index_url (& self) -> & Url { & self . index_url } pub fn api_url (& self) -> & Url { & self . api_url } pub fn token (& self) -> & str { match & self . token { Token :: Plaintext (s) => s , Token :: Keys (_ , _) => panic ! ("registry was not configured with a plaintext token") , } } pub fn key (& self) -> & str { match & self . token { Token :: Plaintext (_) => panic ! ("registry was not configured with a secret key") , Token :: Keys (s , _) => s , } } # [doc = " Shutdown the server thread and wait for it to stop."] # [doc = " `Drop` automatically stops the server, but this additionally"] # [doc = " waits for the thread to stop."] pub fn join (self) { if let Some (mut server) = self . server { server . stop () ; let handle = server . handle . take () . unwrap () ; handle . join () . unwrap () ; } } }
};
}
