// Generated macro for DecryptPacketResult (struct)
macro_rules! Depcrate_connection_packet_cryptoDecryptPacketResult {
() => {
// Module: crate::connection::packet_crypto
// Provides: {"DecryptPacketResult"}
// Dependencies: {}
pub (super) struct DecryptPacketResult { # [doc = " The packet number"] pub (super) number : u64 , # [doc = " Whether a locally initiated key update has been acknowledged by the peer"] pub (super) outgoing_key_update_acked : bool , # [doc = " Whether the peer has initiated a key update"] pub (super) incoming_key_update : bool , }
};
}
