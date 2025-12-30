// Generated macro for read_i32 (function)
macro_rules! Depcrate_decode_sintread_i32 {
() => {
// Module: crate::decode::sint
// Provides: {"read_i32"}
// Dependencies: {}
# [doc = " Attempts to read exactly 5 bytes from the given reader and to decode them as `i32` value."] # [doc = ""] # [doc = " The first byte should be the marker and the others should represent the data itself."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueReadError` on any I/O error while reading either the marker or"] # [doc = " the data."] # [doc = ""] # [doc = " It also returns `ValueReadError::TypeMismatch` if the actual type is not equal with the"] # [doc = " expected one, indicating you with the actual type."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This function will silently retry on every EINTR received from the underlying `Read` until"] # [doc = " successful read."] pub fn read_i32 < R : RmpRead > (rd : & mut R) -> Result < i32 , ValueReadError < R :: Error > > { match read_marker (rd) ? { Marker :: I32 => rd . read_data_i32 () , marker => Err (ValueReadError :: TypeMismatch (marker)) , } }
};
}
