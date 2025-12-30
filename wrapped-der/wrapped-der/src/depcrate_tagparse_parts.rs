// Generated macro for parse_parts (function)
macro_rules! Depcrate_tagparse_parts {
() => {
// Module: crate::tag
// Provides: {"parse_parts"}
// Dependencies: {}
fn parse_parts < 'a , R : Reader < 'a > > (first_byte : u8 , reader : & mut R) -> Result < (bool , TagNumber) > { let constructed = first_byte & CONSTRUCTED_FLAG != 0 ; let first_number_part = first_byte & TagNumber :: MASK ; if first_number_part != TagNumber :: MASK { return Ok ((constructed , TagNumber (first_number_part . into ()))) ; } let mut multi_byte_tag_number = 0 ; for i in 0 .. Tag :: MAX_SIZE - 1 { let byte = reader . read_byte () ? ; multi_byte_tag_number |= u32 :: from (byte & 0x7F) ; if byte & 0x80 == 0 { if multi_byte_tag_number < u32 :: from (TagNumber :: MASK) { return Err (reader . error (ErrorKind :: TagNumberInvalid)) ; } return Ok ((constructed , TagNumber (multi_byte_tag_number))) ; } else if i == 0 && multi_byte_tag_number == 0 { return Err (reader . error (ErrorKind :: TagNumberInvalid)) ; } if multi_byte_tag_number . leading_zeros () < 7 { return Err (reader . error (ErrorKind :: TagNumberInvalid)) ; } multi_byte_tag_number <<= 7 ; } Err (reader . error (ErrorKind :: TagNumberInvalid)) }
};
}
