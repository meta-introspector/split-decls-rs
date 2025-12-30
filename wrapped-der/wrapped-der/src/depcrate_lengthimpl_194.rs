// Generated macro for impl_194 (impl)
macro_rules! Depcrate_lengthimpl_194 {
() => {
// Module: crate::length
// Provides: {"impl_194"}
// Dependencies: {}
impl < 'a > Decode < 'a > for Length { type Error = Error ; fn decode < R : Reader < 'a > > (reader : & mut R) -> Result < Length > { match reader . read_byte () ? { len if len < INDEFINITE_LENGTH_OCTET => Ok (len . into ()) , INDEFINITE_LENGTH_OCTET => match reader . encoding_rules () { # [cfg (feature = "ber")] EncodingRules :: Ber => indefinite :: decode_indefinite_length (& mut reader . clone ()) , EncodingRules :: Der => Err (reader . error (ErrorKind :: IndefiniteLength)) , } , tag @ 0x81 ..= 0x84 => { let nbytes = tag . checked_sub (0x80) . ok_or_else (| | reader . error (ErrorKind :: Overlength)) ? as usize ; debug_assert ! (nbytes <= 4) ; let mut decoded_len = 0u32 ; for _ in 0 .. nbytes { decoded_len = decoded_len . checked_shl (8) . ok_or_else (| | reader . error (ErrorKind :: Overflow)) ? | u32 :: from (reader . read_byte () ?) ; } let length = Length :: from (decoded_len) ; if length . initial_octet () == Some (tag) { Ok (length) } else { Err (reader . error (ErrorKind :: Overlength)) } } _ => { Err (reader . error (ErrorKind :: Overlength)) } } } }
};
}
