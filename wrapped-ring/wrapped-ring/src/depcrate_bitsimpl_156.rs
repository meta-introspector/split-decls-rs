// Generated macro for impl_156 (impl)
macro_rules! Depcrate_bitsimpl_156 {
() => {
// Module: crate::bits
// Provides: {"impl_156"}
// Dependencies: {}
impl FromByteLen < usize > for BitLength < u64 > { # [inline] fn from_byte_len (bytes : usize) -> Result < Self , InputTooLongError < usize > > { match polyfill :: u64_from_usize (bytes) . checked_mul (8) { Some (bits) => Ok (Self (bits)) , None => Err (InputTooLongError :: new (bytes)) , } } }
};
}
