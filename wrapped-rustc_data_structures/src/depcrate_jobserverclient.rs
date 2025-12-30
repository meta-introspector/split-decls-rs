// Generated macro for client (function)
macro_rules! Depcrate_jobserverclient {
() => {
// Module: crate::jobserver
// Provides: {"client"}
// Dependencies: {}
pub fn client () -> Client { GLOBAL_CLIENT_CHECKED . get () . expect (ACCESS_ERROR) . clone () }
};
}
