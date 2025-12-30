// Generated macro for encode_pkt (function)
macro_rules! Depcrate_test_utilsencode_pkt {
() => {
// Module: crate::test_utils
// Provides: {"encode_pkt"}
// Dependencies: {}
pub fn encode_pkt (conn : & mut Connection , pkt_type : Type , frames : & [frame :: Frame] , buf : & mut [u8] ,) -> Result < usize > { let mut b = octets :: OctetsMut :: with_slice (buf) ; let epoch = pkt_type . to_epoch () ? ; let crypto_ctx = & mut conn . crypto_ctx [epoch] ; let pn = conn . next_pkt_num ; let pn_len = 4 ; let send_path = conn . paths . get_active () ? ; let active_dcid_seq = send_path . active_dcid_seq . as_ref () . ok_or (Error :: InvalidState) ? ; let active_scid_seq = send_path . active_scid_seq . as_ref () . ok_or (Error :: InvalidState) ? ; let hdr = Header { ty : pkt_type , version : conn . version , dcid : ConnectionId :: from_ref (conn . ids . get_dcid (* active_dcid_seq) ? . cid . as_ref () ,) , scid : ConnectionId :: from_ref (conn . ids . get_scid (* active_scid_seq) ? . cid . as_ref () ,) , pkt_num : pn , pkt_num_len : pn_len , token : conn . token . clone () , versions : None , key_phase : conn . key_phase , } ; hdr . to_bytes (& mut b) ? ; let payload_len = frames . iter () . fold (0 , | acc , x | acc + x . wire_len ()) ; if pkt_type != Type :: Short { let len = pn_len + payload_len + crypto_ctx . crypto_overhead () . unwrap () ; b . put_varint (len as u64) ? ; } b . put_u32 (pn as u32) ? ; let payload_offset = b . off () ; for frame in frames { frame . to_bytes (& mut b) ? ; } let aead = match crypto_ctx . crypto_seal { Some (ref v) => v , None => return Err (Error :: InvalidState) , } ; let written = packet :: encrypt_pkt (& mut b , pn , pn_len , payload_len , payload_offset , None , aead ,) ? ; conn . next_pkt_num += 1 ; Ok (written) }
};
}
