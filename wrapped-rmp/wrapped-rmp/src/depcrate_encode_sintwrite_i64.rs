// Generated macro for write_i64 (function)
macro_rules! Depcrate_encode_sintwrite_i64 {
() => {
// Module: crate::encode::sint
// Provides: {"write_i64"}
// Dependencies: {}
# [doc = " Encodes and attempts to write an `i64` value as a 9-byte sequence into the given write."] # [doc = ""] # [doc = " The first byte becomes the marker and the others will represent the data itself."] # [doc = ""] # [doc = " Note, that this function will encode the given value in 9-byte sequence no matter what, even if"] # [doc = " the value can be represented using single byte as a fixnum. Also note, that the first byte will"] # [doc = " always be the i16 marker (`0xd3`)."] # [doc = ""] # [doc = " If you need to fit the given buffer efficiently use `write_sint` instead, which automatically"] # [doc = " selects the appropriate integer representation."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueWriteError` on any I/O error occurred while writing either the"] # [doc = " marker or the data."] pub fn write_i64 < W : RmpWrite > (wr : & mut W , val : i64) -> Result < () , ValueWriteError < W :: Error > > { write_marker (wr , Marker :: I64) ? ; wr . write_data_i64 (val) ? ; Ok (()) }
};
}
