// Generated macro for impl_292 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_292 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_292"}
// Dependencies: {}
impl < R > ControlledReader < R > where R : Read , { fn new (reader : R) -> Self { Self { inner : BufReader :: new (BufferedReader :: new (reader)) , } } fn flush_in_buffer (& mut self) { let b = self . inner . buffer () . to_vec () ; self . inner . consume (b . len ()) ; self . keep_in_buffer (& b) ; } }
};
}
