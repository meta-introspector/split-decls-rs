// Generated macro for encode_pkt_num (function)
macro_rules! Depcrate_packetencode_pkt_num {
() => {
// Module: crate::packet
// Provides: {"encode_pkt_num"}
// Dependencies: {}
pub fn encode_pkt_num (pn : u64 , pn_len : usize , b : & mut octets :: OctetsMut ,) -> Result < () > { match pn_len { 1 => b . put_u8 (pn as u8) ? , 2 => b . put_u16 (pn as u16) ? , 3 => b . put_u24 (pn as u32) ? , 4 => b . put_u32 (pn as u32) ? , _ => return Err (Error :: InvalidPacket) , } ; Ok (()) }
};
}
