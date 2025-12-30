// Generated macro for encode_int (function)
macro_rules! Depcrate_hpack_encoderencode_int {
() => {
// Module: crate::hpack::encoder
// Provides: {"encode_int"}
// Dependencies: {}
# [doc = " Encode an integer into the given destination buffer"] fn encode_int < B : BufMut > (mut value : usize , prefix_bits : usize , first_byte : u8 , dst : & mut B ,) { if encode_int_one_byte (value , prefix_bits) { dst . put_u8 (first_byte | value as u8) ; return ; } let low = (1 << prefix_bits) - 1 ; value -= low ; dst . put_u8 (first_byte | low as u8) ; while value >= 128 { dst . put_u8 (0b1000_0000 | value as u8) ; value >>= 7 ; } dst . put_u8 (value as u8) ; }
};
}
