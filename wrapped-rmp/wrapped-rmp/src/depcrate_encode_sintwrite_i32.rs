// Generated macro for write_i32 (function)
macro_rules! Depcrate_encode_sintwrite_i32 {
() => {
// Module: crate::encode::sint
// Provides: {"write_i32"}
// Dependencies: {}
# [doc = " Encodes and attempts to write an `i32` value as a 5-byte sequence into the given write."] # [doc = ""] # [doc = " The first byte becomes the marker and the others will represent the data itself."] # [doc = ""] # [doc = " Note, that this function will encode the given value in 5-byte sequence no matter what, even if"] # [doc = " the value can be represented using single byte as a fixnum. Also note, that the first byte will"] # [doc = " always be the i32 marker (`0xd2`)."] # [doc = ""] # [doc = " If you need to fit the given buffer efficiently use `write_sint` instead, which automatically"] # [doc = " selects the appropriate integer representation."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueWriteError` on any I/O error occurred while writing either the"] # [doc = " marker or the data."] pub fn write_i32 < W : RmpWrite > (wr : & mut W , val : i32) -> Result < () , ValueWriteError < W :: Error > > { write_marker (wr , Marker :: I32) ? ; wr . write_data_i32 (val) ? ; Ok (()) }
};
}
