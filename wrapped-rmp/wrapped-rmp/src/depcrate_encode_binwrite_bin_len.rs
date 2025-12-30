// Generated macro for write_bin_len (function)
macro_rules! Depcrate_encode_binwrite_bin_len {
() => {
// Module: crate::encode::bin
// Provides: {"write_bin_len"}
// Dependencies: {}
# [doc = " Encodes and attempts to write the most efficient binary array length implementation to the given"] # [doc = " write, returning the marker used."] # [doc = ""] # [doc = " This function is useful when you want to get full control for writing the data itself, for"] # [doc = " example, when using non-blocking socket."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueWriteError` on any I/O error occurred while writing either the"] # [doc = " marker or the data."] pub fn write_bin_len < W : RmpWrite > (wr : & mut W , len : u32) -> Result < Marker , ValueWriteError < W :: Error > > { let marker = if len < 256 { Marker :: Bin8 } else if u16 :: try_from (len) . is_ok () { Marker :: Bin16 } else { Marker :: Bin32 } ; write_marker (& mut * wr , marker) ? ; if marker == Marker :: Bin8 { wr . write_data_u8 (len as u8) ? ; } else if marker == Marker :: Bin16 { wr . write_data_u16 (len as u16) ? ; } else if marker == Marker :: Bin32 { wr . write_data_u32 (len) ? ; } Ok (marker) }
};
}
