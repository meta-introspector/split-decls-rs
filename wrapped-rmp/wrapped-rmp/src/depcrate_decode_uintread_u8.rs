// Generated macro for read_u8 (function)
macro_rules! Depcrate_decode_uintread_u8 {
() => {
// Module: crate::decode::uint
// Provides: {"read_u8"}
// Dependencies: {}
# [doc = " Attempts to read exactly 2 bytes from the given reader and to decode them as `u8` value."] # [doc = ""] # [doc = " The first byte should be the marker and the second one should represent the data itself."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueReadError` on any I/O error while reading either the marker or"] # [doc = " the data."] # [doc = ""] # [doc = " It also returns `ValueReadError::TypeMismatch` if the actual type is not equal with the"] # [doc = " expected one, indicating you with the actual type."] pub fn read_u8 < R : RmpRead > (rd : & mut R) -> Result < u8 , ValueReadError < R :: Error > > { match read_marker (rd) ? { Marker :: U8 => rd . read_data_u8 () , marker => Err (ValueReadError :: TypeMismatch (marker)) , } }
};
}
