// Generated macro for negotiate_version (function)
macro_rules! Depcrate_packetnegotiate_version {
() => {
// Module: crate::packet
// Provides: {"negotiate_version"}
// Dependencies: {}
pub fn negotiate_version (scid : & [u8] , dcid : & [u8] , out : & mut [u8] ,) -> Result < usize > { let mut b = octets :: OctetsMut :: with_slice (out) ; let first = rand :: rand_u8 () | FORM_BIT ; b . put_u8 (first) ? ; b . put_u32 (0) ? ; b . put_u8 (scid . len () as u8) ? ; b . put_bytes (scid) ? ; b . put_u8 (dcid . len () as u8) ? ; b . put_bytes (dcid) ? ; b . put_u32 (crate :: PROTOCOL_VERSION_V1) ? ; Ok (b . off ()) }
};
}
