// Generated macro for EscapeBytes (struct)
macro_rules! Depcrate_escape_bytesEscapeBytes {
() => {
// Module: crate::escape_bytes
// Provides: {"EscapeBytes"}
// Dependencies: {}
# [doc = " An iterator of `char` values that represent an escaping of arbitrary bytes."] # [doc = ""] # [doc = " The lifetime parameter `'a` refers to the lifetime of the bytes being"] # [doc = " escaped."] # [doc = ""] # [doc = " This iterator is created by the"] # [doc = " [`ByteSlice::escape_bytes`](crate::ByteSlice::escape_bytes) method."] # [derive (Clone , Debug)] pub struct EscapeBytes < 'a > { remaining : & 'a [u8] , state : EscapeState , }
};
}
