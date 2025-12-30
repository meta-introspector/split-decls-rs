// Generated macro for write_str_len (function)
macro_rules! Depcrate_encode_strwrite_str_len {
() => {
// Module: crate::encode::str
// Provides: {"write_str_len"}
// Dependencies: {}
# [doc = " Encodes and attempts to write the most efficient string length implementation to the given"] # [doc = " write, returning the marker used."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueWriteError` on any I/O error occurred while writing either the"] # [doc = " marker or the data."] pub fn write_str_len < W : RmpWrite > (wr : & mut W , len : u32) -> Result < Marker , ValueWriteError < W :: Error > > { let marker = if len < 32 { Marker :: FixStr (len as u8) } else if len < 256 { Marker :: Str8 } else if u16 :: try_from (len) . is_ok () { Marker :: Str16 } else { Marker :: Str32 } ; write_marker (wr , marker) ? ; if marker == Marker :: Str8 { wr . write_data_u8 (len as u8) ? ; } if marker == Marker :: Str16 { wr . write_data_u16 (len as u16) ? ; } if marker == Marker :: Str32 { wr . write_data_u32 (len) ? ; } Ok (marker) }
};
}
