// Generated macro for read_i64 (function)
macro_rules! Depcrate_decode_sintread_i64 {
() => {
// Module: crate::decode::sint
// Provides: {"read_i64"}
// Dependencies: {}
# [doc = " Attempts to read exactly 9 bytes from the given reader and to decode them as `i64` value."] # [doc = ""] # [doc = " The first byte should be the marker and the others should represent the data itself."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueReadError` on any I/O error while reading either the marker or"] # [doc = " the data."] # [doc = ""] # [doc = " It also returns `ValueReadError::TypeMismatch` if the actual type is not equal with the"] # [doc = " expected one, indicating you with the actual type."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This function will silently retry on every EINTR received from the underlying `Read` until"] # [doc = " successful read."] pub fn read_i64 < R : RmpRead > (rd : & mut R) -> Result < i64 , ValueReadError < R :: Error > > { match read_marker (rd) ? { Marker :: I64 => rd . read_data_i64 () , marker => Err (ValueReadError :: TypeMismatch (marker)) , } }
};
}
