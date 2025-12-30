// Generated macro for decode_pkt (function)
macro_rules! Depcrate_test_utilsdecode_pkt {
() => {
// Module: crate::test_utils
// Provides: {"decode_pkt"}
// Dependencies: {}
pub fn decode_pkt (conn : & mut Connection , buf : & mut [u8] ,) -> Result < Vec < frame :: Frame > > { let mut b = octets :: OctetsMut :: with_slice (buf) ; let mut hdr = Header :: from_bytes (& mut b , conn . source_id () . len ()) . unwrap () ; let epoch = hdr . ty . to_epoch () ? ; let aead = conn . crypto_ctx [epoch] . crypto_open . as_ref () . unwrap () ; let payload_len = b . cap () ; packet :: decrypt_hdr (& mut b , & mut hdr , aead) . unwrap () ; let pn = packet :: decode_pkt_num (conn . pkt_num_spaces [epoch] . largest_rx_pkt_num , hdr . pkt_num , hdr . pkt_num_len ,) ; let mut payload = packet :: decrypt_pkt (& mut b , pn , hdr . pkt_num_len , payload_len , aead) . unwrap () ; let mut frames = Vec :: new () ; while payload . cap () > 0 { let frame = frame :: Frame :: from_bytes (& mut payload , hdr . ty) ? ; frames . push (frame) ; } Ok (frames) }
};
}
