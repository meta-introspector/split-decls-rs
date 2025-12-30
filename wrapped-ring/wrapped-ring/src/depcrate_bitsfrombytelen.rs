// Generated macro for FromByteLen (trait)
macro_rules! Depcrate_bitsFromByteLen {
() => {
// Module: crate::bits
// Provides: {"FromByteLen"}
// Dependencies: {}
pub (crate) trait FromByteLen < T > : Sized { # [doc = " Constructs a `BitLength` from the given length in bytes."] # [doc = ""] # [doc = " Fails if `bytes * 8` is too large for a `T`."] fn from_byte_len (bytes : T) -> Result < Self , InputTooLongError < T > > ; }
};
}
