// Generated macro for read_bool (function)
macro_rules! Depcrate_decoderead_bool {
() => {
// Module: crate::decode
// Provides: {"read_bool"}
// Dependencies: {}
# [doc = " Attempts to read a single byte from the given reader and to decode it as a boolean value."] # [doc = ""] # [doc = " According to the MessagePack specification, an encoded boolean value is represented as a single"] # [doc = " byte."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueReadError` on any I/O error while reading the bool marker,"] # [doc = " except the EINTR, which is handled internally."] # [doc = ""] # [doc = " It also returns `ValueReadError::TypeMismatch` if the actual type is not equal with the"] # [doc = " expected one, indicating you with the actual type."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This function will silently retry on every EINTR received from the underlying `Read` until"] # [doc = " successful read."] pub fn read_bool < R : RmpRead > (rd : & mut R) -> Result < bool , ValueReadError < R :: Error > > { match read_marker (rd) ? { Marker :: True => Ok (true) , Marker :: False => Ok (false) , marker => Err (ValueReadError :: TypeMismatch (marker)) , } }
};
}
