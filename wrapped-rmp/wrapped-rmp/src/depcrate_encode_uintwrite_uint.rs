// Generated macro for write_uint (function)
macro_rules! Depcrate_encode_uintwrite_uint {
() => {
// Module: crate::encode::uint
// Provides: {"write_uint"}
// Dependencies: {}
# [doc = " Encodes and attempts to write an `u64` value into the given write using the most efficient"] # [doc = " representation, returning the marker used."] # [doc = ""] # [doc = " This function obeys the MessagePack specification, which requires that the serializer SHOULD use"] # [doc = " the format which represents the data in the smallest number of bytes."] # [doc = ""] # [doc = " The first byte becomes the marker and the others (if present, up to 9) will represent the data"] # [doc = " itself."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueWriteError` on any I/O error occurred while writing either the"] # [doc = " marker or the data."] pub fn write_uint < W : RmpWrite > (wr : & mut W , val : u64) -> Result < Marker , ValueWriteError < W :: Error > > { if val < 256 { write_uint8 (wr , val as u8) } else if val < 65536 { write_u16 (wr , val as u16) . and (Ok (Marker :: U16)) } else if val < 4294967296 { write_u32 (wr , val as u32) . and (Ok (Marker :: U32)) } else { write_u64 (wr , val) . and (Ok (Marker :: U64)) } }
};
}
