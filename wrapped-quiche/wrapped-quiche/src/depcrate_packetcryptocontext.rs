// Generated macro for CryptoContext (struct)
macro_rules! Depcrate_packetCryptoContext {
() => {
// Module: crate::packet
// Provides: {"CryptoContext"}
// Dependencies: {}
pub struct CryptoContext { pub key_update : Option < KeyUpdate > , pub crypto_open : Option < crypto :: Open > , pub crypto_seal : Option < crypto :: Seal > , pub crypto_0rtt_open : Option < crypto :: Open > , pub crypto_stream : stream :: Stream , }
};
}
