// Generated macro for client (function)
macro_rules! Depcrateclient {
() => {
// Module: crate
// Provides: {"client"}
// Dependencies: {}
fn client (conn : & mut Connection) -> & mut ClientConnection { conn . try_into () . unwrap () }
};
}
