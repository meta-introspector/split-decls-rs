// Generated macro for write_i8 (function)
macro_rules! Depcrate_encode_sintwrite_i8 {
() => {
// Module: crate::encode::sint
// Provides: {"write_i8"}
// Dependencies: {}
# [doc = " Encodes and attempts to write an `i8` value as a 2-byte sequence into the given write."] # [doc = ""] # [doc = " The first byte becomes the marker and the second one will represent the data itself."] # [doc = ""] # [doc = " Note, that this function will encode the given value in 2-byte sequence no matter what, even if"] # [doc = " the value can be represented using single byte as a fixnum. Also note, that the first byte will"] # [doc = " always be the i8 marker (`0xd0`)."] # [doc = ""] # [doc = " If you need to fit the given buffer efficiently use `write_sint` instead, which automatically"] # [doc = " selects the appropriate integer representation."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueWriteError` on any I/O error occurred while writing either the"] # [doc = " marker or the data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let mut buf = [0x00, 0x00];"] # [doc = ""] # [doc = " rmp::encode::write_i8(&mut &mut buf[..], 42).ok().unwrap();"] # [doc = " assert_eq!([0xd0, 0x2a], buf);"] # [doc = ""] # [doc = " // Note, that -18 can be represented simply as `[0xee]`, but the function emits 2-byte sequence."] # [doc = " rmp::encode::write_i8(&mut &mut buf[..], -18).ok().unwrap();"] # [doc = " assert_eq!([0xd0, 0xee], buf);"] # [doc = " ```"] pub fn write_i8 < W : RmpWrite > (wr : & mut W , val : i8) -> Result < () , ValueWriteError < W :: Error > > { write_marker (wr , Marker :: I8) ? ; wr . write_data_i8 (val) ? ; Ok (()) }
};
}
