// Generated macro for UnprotectHeaderResult (struct)
macro_rules! Depcrate_connection_packet_cryptoUnprotectHeaderResult {
() => {
// Module: crate::connection::packet_crypto
// Provides: {"UnprotectHeaderResult"}
// Dependencies: {}
pub (super) struct UnprotectHeaderResult { # [doc = " The packet with the now unprotected header (`None` in the case of stateless reset packets"] # [doc = " that fail to be decoded)"] pub (super) packet : Option < Packet > , # [doc = " Whether the packet was a stateless reset packet"] pub (super) stateless_reset : bool , }
};
}
