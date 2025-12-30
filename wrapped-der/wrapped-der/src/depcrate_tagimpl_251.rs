// Generated macro for impl_251 (impl)
macro_rules! Depcrate_tagimpl_251 {
() => {
// Module: crate::tag
// Provides: {"impl_251"}
// Dependencies: {}
impl Encode for Tag { # [allow (clippy :: cast_possible_truncation)] fn encoded_len (& self) -> Result < Length > { let number = self . number () . value () ; let length = if number <= 30 { Length :: ONE } else { Length :: new (number . ilog2 () / 7 + 2) } ; Ok (length) } fn encode (& self , writer : & mut impl Writer) -> Result < () > { let mut first_byte = (self . class () as u8) | (u8 :: from (self . is_constructed ()) << 5) ; let number = self . number () . value () ; if number < u32 :: from (TagNumber :: MASK) { first_byte |= (number & 0x1F) as u8 ; writer . write_byte (first_byte) ? ; } else { first_byte |= TagNumber :: MASK ; writer . write_byte (first_byte) ? ; let extra_bytes = number . ilog2 () / 7 + 1 ; for shift in (0 .. extra_bytes) . rev () { let mut byte = ((number >> (shift * 7)) & 0x7f) as u8 ; if shift != 0 { byte |= 0x80 ; } writer . write_byte (byte) ? ; } } Ok (()) } }
};
}
