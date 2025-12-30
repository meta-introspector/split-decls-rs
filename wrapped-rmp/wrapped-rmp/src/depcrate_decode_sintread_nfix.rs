// Generated macro for read_nfix (function)
macro_rules! Depcrate_decode_sintread_nfix {
() => {
// Module: crate::decode::sint
// Provides: {"read_nfix"}
// Dependencies: {}
# [doc = " Attempts to read a single byte from the given reader and to decode it as a negative fixnum"] # [doc = " value."] # [doc = ""] # [doc = " According to the MessagePack specification, a negative fixed integer value is represented using"] # [doc = " a single byte in `[0xe0; 0xff]` range inclusively, prepended with a special marker mask."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueReadError` on any I/O error while reading the marker,"] # [doc = " except the EINTR, which is handled internally."] # [doc = ""] # [doc = " It also returns `ValueReadError::TypeMismatch` if the actual type is not equal with the"] # [doc = " expected one, indicating you with the actual type."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This function will silently retry on every EINTR received from the underlying `Read` until"] # [doc = " successful read."] pub fn read_nfix < R : RmpRead > (rd : & mut R) -> Result < i8 , ValueReadError < R :: Error > > { match read_marker (rd) ? { Marker :: FixNeg (val) => Ok (val) , marker => Err (ValueReadError :: TypeMismatch (marker)) , } }
};
}
