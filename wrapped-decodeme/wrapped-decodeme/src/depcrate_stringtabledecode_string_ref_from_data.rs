// Generated macro for decode_string_ref_from_data (function)
macro_rules! Depcrate_stringtabledecode_string_ref_from_data {
() => {
// Module: crate::stringtable
// Provides: {"decode_string_ref_from_data"}
// Dependencies: {}
fn decode_string_ref_from_data (bytes : & [u8]) -> StringId { assert ! (bytes [0] == STRING_REF_TAG) ; assert ! (STRING_REF_ENCODED_SIZE == 9) ; let id = u64 :: from_le_bytes (bytes [1 .. 9] . try_into () . unwrap ()) ; StringId :: new (id) }
};
}
