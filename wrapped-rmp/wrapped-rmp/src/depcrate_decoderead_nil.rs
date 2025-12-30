// Generated macro for read_nil (function)
macro_rules! Depcrate_decoderead_nil {
() => {
// Module: crate::decode
// Provides: {"read_nil"}
// Dependencies: {}
# [doc = " Attempts to read a single byte from the given reader and to decode it as a nil value."] # [doc = ""] # [doc = " According to the MessagePack specification, a nil value is represented as a single `0xc0` byte."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueReadError` on any I/O error while reading the nil marker,"] # [doc = " except the EINTR, which is handled internally."] # [doc = ""] # [doc = " It also returns `ValueReadError::TypeMismatch` if the actual type is not equal with the"] # [doc = " expected one, indicating you with the actual type."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This function will silently retry on every EINTR received from the underlying `Read` until"] # [doc = " successful read."] pub fn read_nil < R : RmpRead > (rd : & mut R) -> Result < () , ValueReadError < R :: Error > > { match read_marker (rd) ? { Marker :: Null => Ok (()) , marker => Err (ValueReadError :: TypeMismatch (marker)) , } }
};
}
