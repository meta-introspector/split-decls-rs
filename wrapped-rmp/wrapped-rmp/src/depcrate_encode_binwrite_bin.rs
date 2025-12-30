// Generated macro for write_bin (function)
macro_rules! Depcrate_encode_binwrite_bin {
() => {
// Module: crate::encode::bin
// Provides: {"write_bin"}
// Dependencies: {}
# [doc = " Encodes and attempts to write the most efficient binary implementation to the given `Write`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueWriteError` on any I/O error occurred while writing either the"] # [doc = " marker or the data."] pub fn write_bin < W : RmpWrite > (wr : & mut W , data : & [u8]) -> Result < () , ValueWriteError < W :: Error > > { write_bin_len (wr , data . len () as u32) ? ; wr . write_bytes (data) . map_err (ValueWriteError :: InvalidDataWrite) }
};
}
