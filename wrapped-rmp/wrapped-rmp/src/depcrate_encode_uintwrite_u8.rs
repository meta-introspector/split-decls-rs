// Generated macro for write_u8 (function)
macro_rules! Depcrate_encode_uintwrite_u8 {
() => {
// Module: crate::encode::uint
// Provides: {"write_u8"}
// Dependencies: {}
# [doc = " Encodes and attempts to write an `u8` value as a 2-byte sequence into the given write."] # [doc = ""] # [doc = " The first byte becomes the marker and the second one will represent the data itself."] # [doc = ""] # [doc = " Note, that this function will encode the given value in 2-byte sequence no matter what, even if"] # [doc = " the value can be represented using single byte as a positive fixnum."] # [doc = ""] # [doc = " If you need to fit the given buffer efficiently use `write_uint` instead, which automatically"] # [doc = " selects the appropriate integer representation."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueWriteError` on any I/O error occurred while writing either the"] # [doc = " marker or the data."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " let mut buf = [0x00, 0x00];"] # [doc = ""] # [doc = " rmp::encode::write_u8(&mut &mut buf[..], 146).ok().unwrap();"] # [doc = " assert_eq!([0xcc, 0x92], buf);"] # [doc = ""] # [doc = " // Note, that 42 can be represented simply as `[0x2a]`, but the function emits 2-byte sequence."] # [doc = " rmp::encode::write_u8(&mut &mut buf[..], 42).ok().unwrap();"] # [doc = " assert_eq!([0xcc, 0x2a], buf);"] # [doc = " ```"] pub fn write_u8 < W : RmpWrite > (wr : & mut W , val : u8) -> Result < () , ValueWriteError < W :: Error > > { write_marker (wr , Marker :: U8) ? ; wr . write_data_u8 (val) ? ; Ok (()) }
};
}
