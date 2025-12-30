// Generated macro for write_ext_meta (function)
macro_rules! Depcrate_encodewrite_ext_meta {
() => {
// Module: crate::encode
// Provides: {"write_ext_meta"}
// Dependencies: {}
# [doc = " Encodes and attempts to write the most efficient ext metadata implementation to the given"] # [doc = " write, returning the marker used."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueWriteError` on any I/O error occurred while writing either the"] # [doc = " marker or the data."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `ty` is negative, because it is reserved for future MessagePack extension including"] # [doc = " 2-byte type information."] pub fn write_ext_meta < W : RmpWrite > (wr : & mut W , len : u32 , ty : i8) -> Result < Marker , ValueWriteError < W :: Error > > { let marker = match len { 1 => Marker :: FixExt1 , 2 => Marker :: FixExt2 , 4 => Marker :: FixExt4 , 8 => Marker :: FixExt8 , 16 => Marker :: FixExt16 , 0 ..= 255 => Marker :: Ext8 , 256 ..= 65535 => Marker :: Ext16 , _ => Marker :: Ext32 , } ; write_marker (wr , marker) ? ; if marker == Marker :: Ext8 { wr . write_data_u8 (len as u8) ? ; } else if marker == Marker :: Ext16 { wr . write_data_u16 (len as u16) ? ; } else if marker == Marker :: Ext32 { wr . write_data_u32 (len) ? ; } wr . write_data_i8 (ty) ? ; Ok (marker) }
};
}
