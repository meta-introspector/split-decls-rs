// Generated macro for impl_296 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_296 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_296"}
// Dependencies: {}
impl < R > Read for BufferedReader < R > where R : Read , { fn read (& mut self , mut buf : & mut [u8]) -> io :: Result < usize > { if self . buffer . is_empty () { self . inner . read (buf) } else { let n = buf . write (& self . buffer) ? ; let _ = self . buffer . drain (.. n) ; Ok (n) } } }
};
}
