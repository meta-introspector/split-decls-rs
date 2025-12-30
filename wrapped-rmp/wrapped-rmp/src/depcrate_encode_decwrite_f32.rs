// Generated macro for write_f32 (function)
macro_rules! Depcrate_encode_decwrite_f32 {
() => {
// Module: crate::encode::dec
// Provides: {"write_f32"}
// Dependencies: {}
# [doc = " Encodes and attempts to write an `f32` value as a 5-byte sequence into the given write."] # [doc = ""] # [doc = " The first byte becomes the `f32` marker and the others will represent the data itself."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueWriteError` on any I/O error occurred while writing either the"] # [doc = " marker or the data."] pub fn write_f32 < W : RmpWrite > (wr : & mut W , val : f32) -> Result < () , ValueWriteError < W :: Error > > { write_marker (wr , Marker :: F32) ? ; wr . write_data_f32 (val) ? ; Ok (()) }
};
}
