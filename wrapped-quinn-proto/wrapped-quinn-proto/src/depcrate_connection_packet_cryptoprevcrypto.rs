// Generated macro for PrevCrypto (struct)
macro_rules! Depcrate_connection_packet_cryptoPrevCrypto {
() => {
// Module: crate::connection::packet_crypto
// Provides: {"PrevCrypto"}
// Dependencies: {}
pub (super) struct PrevCrypto { # [doc = " The keys used for the previous key phase, temporarily retained to decrypt packets sent by"] # [doc = " the peer prior to its own key update."] pub (super) crypto : KeyPair < Box < dyn PacketKey > > , # [doc = " The incoming packet that ends the interval for which these keys are applicable, and the time"] # [doc = " of its receipt."] # [doc = ""] # [doc = " Incoming packets should be decrypted using these keys iff this is `None` or their packet"] # [doc = " number is lower. `None` indicates that we have not yet received a packet using newer keys,"] # [doc = " which implies that the update was locally initiated."] pub (super) end_packet : Option < (u64 , Instant) > , # [doc = " Whether the following key phase is from a remotely initiated update that we haven't acked"] pub (super) update_unacked : bool , }
};
}
