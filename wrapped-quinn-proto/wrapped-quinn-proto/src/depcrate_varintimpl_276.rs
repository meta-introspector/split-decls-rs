// Generated macro for impl_276 (impl)
macro_rules! Depcrate_varintimpl_276 {
() => {
// Module: crate::varint
// Provides: {"impl_276"}
// Dependencies: {}
impl Codec for VarInt { fn decode < B : Buf > (r : & mut B) -> coding :: Result < Self > { if ! r . has_remaining () { return Err (UnexpectedEnd) ; } let mut buf = [0 ; 8] ; buf [0] = r . get_u8 () ; let tag = buf [0] >> 6 ; buf [0] &= 0b0011_1111 ; let x = match tag { 0b00 => u64 :: from (buf [0]) , 0b01 => { if r . remaining () < 1 { return Err (UnexpectedEnd) ; } r . copy_to_slice (& mut buf [1 .. 2]) ; u64 :: from (u16 :: from_be_bytes (buf [.. 2] . try_into () . unwrap ())) } 0b10 => { if r . remaining () < 3 { return Err (UnexpectedEnd) ; } r . copy_to_slice (& mut buf [1 .. 4]) ; u64 :: from (u32 :: from_be_bytes (buf [.. 4] . try_into () . unwrap ())) } 0b11 => { if r . remaining () < 7 { return Err (UnexpectedEnd) ; } r . copy_to_slice (& mut buf [1 .. 8]) ; u64 :: from_be_bytes (buf) } _ => unreachable ! () , } ; Ok (Self (x)) } fn encode < B : BufMut > (& self , w : & mut B) { let x = self . 0 ; if x < 2u64 . pow (6) { w . put_u8 (x as u8) ; } else if x < 2u64 . pow (14) { w . put_u16 ((0b01 << 14) | x as u16) ; } else if x < 2u64 . pow (30) { w . put_u32 ((0b10 << 30) | x as u32) ; } else if x < 2u64 . pow (62) { w . put_u64 ((0b11 << 62) | x) ; } else { unreachable ! ("malformed VarInt") } } }
};
}
