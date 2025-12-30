// Generated macro for impl_511 (impl)
macro_rules! Depcrate_bnimpl_511 {
() => {
// Module: crate::bn
// Provides: {"impl_511"}
// Dependencies: {}
impl TryFrom < & [u8] > for DetachableLcPtr < BIGNUM > { type Error = () ; fn try_from (bytes : & [u8]) -> Result < Self , Self :: Error > { unsafe { DetachableLcPtr :: new (BN_bin2bn (bytes . as_ptr () , bytes . len () , null_mut ())) } } }
};
}
