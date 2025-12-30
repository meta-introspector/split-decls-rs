// Generated macro for retry (function)
macro_rules! Depcrate_packetretry {
() => {
// Module: crate::packet
// Provides: {"retry"}
// Dependencies: {}
pub fn retry (scid : & [u8] , dcid : & [u8] , new_scid : & [u8] , token : & [u8] , version : u32 , out : & mut [u8] ,) -> Result < usize > { let mut b = octets :: OctetsMut :: with_slice (out) ; if ! crate :: version_is_supported (version) { return Err (Error :: UnknownVersion) ; } let hdr = Header { ty : Type :: Retry , version , dcid : ConnectionId :: from_ref (scid) , scid : ConnectionId :: from_ref (new_scid) , pkt_num : 0 , pkt_num_len : 0 , token : Some (token . to_vec ()) , versions : None , key_phase : false , } ; hdr . to_bytes (& mut b) ? ; let tag = compute_retry_integrity_tag (& b , dcid , version) ? ; b . put_bytes (tag . as_ref ()) ? ; Ok (b . off ()) }
};
}
