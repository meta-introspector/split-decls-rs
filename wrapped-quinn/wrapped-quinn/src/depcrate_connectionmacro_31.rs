// Generated macro for macro_31 (macro)
macro_rules! Depcrate_connectionmacro_31 {
() => {
// Module: crate::connection
// Provides: {"macro_31"}
// Dependencies: {}
pin_project ! { # [doc = " Future produced by [`Connection::read_datagram`]"] pub struct ReadDatagram <'a > { conn : &'a ConnectionRef , # [pin] notify : Notified <'a >, } }
};
}
