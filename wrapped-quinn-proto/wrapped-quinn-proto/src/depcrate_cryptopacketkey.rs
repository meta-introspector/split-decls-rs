// Generated macro for PacketKey (trait)
macro_rules! Depcrate_cryptoPacketKey {
() => {
// Module: crate::crypto
// Provides: {"PacketKey"}
// Dependencies: {}
# [doc = " Keys used to protect packet payloads"] pub trait PacketKey : Send + Sync { # [doc = " Encrypt the packet payload with the given packet number"] fn encrypt (& self , packet : u64 , buf : & mut [u8] , header_len : usize) ; # [doc = " Decrypt the packet payload with the given packet number"] fn decrypt (& self , packet : u64 , header : & [u8] , payload : & mut BytesMut ,) -> Result < () , CryptoError > ; # [doc = " The length of the AEAD tag appended to packets on encryption"] fn tag_len (& self) -> usize ; # [doc = " Maximum number of packets that may be sent using a single key"] fn confidentiality_limit (& self) -> u64 ; # [doc = " Maximum number of incoming packets that may fail decryption before the connection must be"] # [doc = " abandoned"] fn integrity_limit (& self) -> u64 ; }
};
}
