// Generated macro for impl_45 (impl)
macro_rules! Depcrate_noprotectionimpl_45 {
() => {
// Module: crate::noprotection
// Provides: {"impl_45"}
// Dependencies: {}
impl crypto :: PacketKey for NoProtectionPacketKey { fn encrypt (& self , _packet : u64 , buf : & mut [u8] , header_len : usize) { let (_header , payload_tag) = buf . split_at_mut (header_len) ; let (_payload , tag_storage) = payload_tag . split_at_mut (payload_tag . len () - self . inner . tag_len ()) ; tag_storage . fill (42) ; } fn decrypt (& self , _packet : u64 , _header : & [u8] , payload : & mut BytesMut ,) -> Result < () , CryptoError > { let plain_len = payload . len () - self . inner . tag_len () ; payload . truncate (plain_len) ; Ok (()) } fn tag_len (& self) -> usize { self . inner . tag_len () } fn confidentiality_limit (& self) -> u64 { self . inner . confidentiality_limit () } fn integrity_limit (& self) -> u64 { self . inner . integrity_limit () } }
};
}
