// Generated macro for macro_33 (macro)
macro_rules! Depcrate_connectionmacro_33 {
() => {
// Module: crate::connection
// Provides: {"macro_33"}
// Dependencies: {}
pin_project ! { # [doc = " Future produced by [`Connection::send_datagram_wait`]"] pub struct SendDatagram <'a > { conn : &'a ConnectionRef , data : Option < Bytes >, # [pin] notify : Notified <'a >, } }
};
}
