// Generated macro for decode (function)
macro_rules! Depcrate_hexdecode {
() => {
// Module: crate::hex
// Provides: {"decode"}
// Dependencies: {}
# [doc = " Converts a hex string to a vector of bytes"] # [doc = " # Errors"] # [doc = " Returns an error if `hex_str` contains a non-hex digit."] # [allow (clippy :: missing_panics_doc)] pub fn decode (hex_str : & str) -> Result < Vec < u8 > , String > { let mut bytes = Vec :: < u8 > :: with_capacity (hex_str . len () / 2 + 1) ; let mut current_byte = b'\0' ; let mut index : u32 = 0 ; for ch in hex_str . chars () { if ! ch . is_ascii_hexdigit () { return Err ("Invalid hex string" . to_string ()) ; } # [allow (clippy :: cast_possible_truncation)] let value = ch . to_digit (16) . unwrap () as u8 ; if index % 2 == 0 { current_byte = value << 4 ; } else { current_byte |= value ; bytes . push (current_byte) ; } if let Some (idx) = index . checked_add (1) { index = idx ; } else { break ; } } if index % 2 == 1 { bytes . push (current_byte) ; } Ok (bytes) }
};
}
