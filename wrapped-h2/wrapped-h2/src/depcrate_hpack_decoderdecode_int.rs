// Generated macro for decode_int (function)
macro_rules! Depcrate_hpack_decoderdecode_int {
() => {
// Module: crate::hpack::decoder
// Provides: {"decode_int"}
// Dependencies: {}
fn decode_int < B : Buf > (buf : & mut B , prefix_size : u8) -> Result < usize , DecoderError > { const MAX_BYTES : usize = 5 ; const VARINT_MASK : u8 = 0b0111_1111 ; const VARINT_FLAG : u8 = 0b1000_0000 ; if prefix_size < 1 || prefix_size > 8 { return Err (DecoderError :: InvalidIntegerPrefix) ; } if ! buf . has_remaining () { return Err (DecoderError :: NeedMore (NeedMore :: IntegerUnderflow)) ; } let mask = if prefix_size == 8 { 0xFF } else { (1u8 << prefix_size) . wrapping_sub (1) } ; let mut ret = (buf . get_u8 () & mask) as usize ; if ret < mask as usize { return Ok (ret) ; } let mut bytes = 1 ; let mut shift = 0 ; while buf . has_remaining () { let b = buf . get_u8 () ; bytes += 1 ; ret += ((b & VARINT_MASK) as usize) << shift ; shift += 7 ; if b & VARINT_FLAG == 0 { return Ok (ret) ; } if bytes == MAX_BYTES { return Err (DecoderError :: IntegerOverflow) ; } } Err (DecoderError :: NeedMore (NeedMore :: IntegerUnderflow)) }
};
}
