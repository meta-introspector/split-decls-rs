// Generated macro for write_uint8 (function)
macro_rules! Depcrate_encode_uintwrite_uint8 {
() => {
// Module: crate::encode::uint
// Provides: {"write_uint8"}
// Dependencies: {}
# [doc = " Encodes and attempts to write an `u8` value into the given write using the most efficient"] # [doc = " representation, returning the marker used."] # [doc = ""] # [doc = " See [`write_uint`] for more info."] pub fn write_uint8 < W : RmpWrite > (wr : & mut W , val : u8) -> Result < Marker , ValueWriteError < W :: Error > > { if val < 128 { write_pfix (wr , val) . and (Ok (Marker :: FixPos (val))) . map_err (ValueWriteError :: InvalidMarkerWrite) } else { write_u8 (wr , val) . and (Ok (Marker :: U8)) } }
};
}
