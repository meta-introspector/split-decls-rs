// Generated macro for Keys (struct)
macro_rules! Depcrate_cryptoKeys {
() => {
// Module: crate::crypto
// Provides: {"Keys"}
// Dependencies: {}
# [doc = " A complete set of keys for a certain packet space"] pub struct Keys { # [doc = " Header protection keys"] pub header : KeyPair < Box < dyn HeaderKey > > , # [doc = " Packet protection keys"] pub packet : KeyPair < Box < dyn PacketKey > > , }
};
}
