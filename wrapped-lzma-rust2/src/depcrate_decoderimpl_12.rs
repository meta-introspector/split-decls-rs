// Generated macro for impl_12 (impl)
macro_rules! Depcrate_decoderimpl_12 {
() => {
// Module: crate::decoder
// Provides: {"impl_12"}
// Dependencies: {}
impl LengthCoder { fn decode < R : RangeReader > (& mut self , pos_state : usize , rc : & mut RangeDecoder < R >) -> i32 { if rc . decode_bit (& mut self . choice [0]) == 0 { return rc . decode_bit_tree (& mut self . low [pos_state]) . wrapping_add (MATCH_LEN_MIN as _) ; } if rc . decode_bit (& mut self . choice [1]) == 0 { return rc . decode_bit_tree (& mut self . mid [pos_state]) . wrapping_add (MATCH_LEN_MIN as _) . wrapping_add (LOW_SYMBOLS as _) ; } rc . decode_bit_tree (& mut self . high) . wrapping_add (MATCH_LEN_MIN as _) . wrapping_add (LOW_SYMBOLS as _) . wrapping_add (MID_SYMBOLS as _) } }
};
}
