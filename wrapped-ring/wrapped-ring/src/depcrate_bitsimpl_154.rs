// Generated macro for impl_154 (impl)
macro_rules! Depcrate_bitsimpl_154 {
() => {
// Module: crate::bits
// Provides: {"impl_154"}
// Dependencies: {}
impl FromByteLen < usize > for BitLength < usize > { # [inline] fn from_byte_len (bytes : usize) -> Result < Self , InputTooLongError > { match bytes . checked_mul (8) { Some (bits) => Ok (Self (bits)) , None => Err (InputTooLongError :: new (bytes)) , } } }
};
}
