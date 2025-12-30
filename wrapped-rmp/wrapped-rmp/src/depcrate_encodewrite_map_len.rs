// Generated macro for write_map_len (function)
macro_rules! Depcrate_encodewrite_map_len {
() => {
// Module: crate::encode
// Provides: {"write_map_len"}
// Dependencies: {}
# [doc = " Encodes and attempts to write the most efficient map length implementation to the given write,"] # [doc = " returning the marker used."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueWriteError` on any I/O error occurred while writing either the"] # [doc = " marker or the data."] pub fn write_map_len < W : RmpWrite > (wr : & mut W , len : u32) -> Result < Marker , ValueWriteError < W :: Error > > { let marker = if len < 16 { Marker :: FixMap (len as u8) } else if u16 :: try_from (len) . is_ok () { Marker :: Map16 } else { Marker :: Map32 } ; write_marker (wr , marker) ? ; if marker == Marker :: Map16 { wr . write_data_u16 (len as u16) ? ; } else if marker == Marker :: Map32 { wr . write_data_u32 (len) ? ; } Ok (marker) }
};
}
