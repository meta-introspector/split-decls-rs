// Generated macro for KeyUpdate (struct)
macro_rules! Depcrate_packetKeyUpdate {
() => {
// Module: crate::packet
// Provides: {"KeyUpdate"}
// Dependencies: {}
pub struct KeyUpdate { # [doc = " 1-RTT key used prior to a key update."] pub crypto_open : crypto :: Open , # [doc = " The packet number triggered the latest key-update."] # [doc = ""] # [doc = " Incoming packets with lower pn should use this (prev) crypto key."] pub pn_on_update : u64 , # [doc = " Whether ACK frame for key-update has been sent."] pub update_acked : bool , # [doc = " When the old key should be discarded."] pub timer : Instant , }
};
}
