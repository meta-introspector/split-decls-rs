// Generated macro for impl_830 (impl)
macro_rules! Depcrate_signimpl_830 {
() => {
// Module: crate::sign
// Provides: {"impl_830"}
// Dependencies: {}
impl RsaPssSaltlen { # [doc = " Returns the integer representation of `RsaPssSaltlen`."] pub (crate) fn as_raw (& self) -> c_int { self . 0 } # [doc = " Sets the salt length to the given value."] pub fn custom (val : c_int) -> RsaPssSaltlen { RsaPssSaltlen (val) } # [doc = " The salt length is set to the digest length."] # [doc = " Corresponds to the special value `-1`."] pub const DIGEST_LENGTH : RsaPssSaltlen = RsaPssSaltlen (- 1) ; # [doc = " The salt length is set to the maximum permissible value."] # [doc = " Corresponds to the special value `-2`."] pub const MAXIMUM_LENGTH : RsaPssSaltlen = RsaPssSaltlen (- 2) ; }
};
}
