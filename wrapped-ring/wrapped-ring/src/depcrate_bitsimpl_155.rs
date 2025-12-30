// Generated macro for impl_155 (impl)
macro_rules! Depcrate_bitsimpl_155 {
() => {
// Module: crate::bits
// Provides: {"impl_155"}
// Dependencies: {}
impl FromByteLen < u64 > for BitLength < u64 > { # [inline] fn from_byte_len (bytes : u64) -> Result < Self , InputTooLongError < u64 > > { match bytes . checked_mul (8) { Some (bits) => Ok (Self (bits)) , None => Err (InputTooLongError :: new (bytes)) , } } }
};
}
