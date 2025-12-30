// Generated macro for impl_510 (impl)
macro_rules! Depcrate_bnimpl_510 {
() => {
// Module: crate::bn
// Provides: {"impl_510"}
// Dependencies: {}
impl TryFrom < & [u8] > for LcPtr < BIGNUM > { type Error = () ; fn try_from (bytes : & [u8]) -> Result < Self , Self :: Error > { unsafe { LcPtr :: new (BN_bin2bn (bytes . as_ptr () , bytes . len () , null_mut ())) } } }
};
}
