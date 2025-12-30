// Generated macro for impl_512 (impl)
macro_rules! Depcrate_bnimpl_512 {
() => {
// Module: crate::bn
// Provides: {"impl_512"}
// Dependencies: {}
impl TryFrom < u64 > for DetachableLcPtr < BIGNUM > { type Error = () ; fn try_from (value : u64) -> Result < Self , Self :: Error > { unsafe { let bn = DetachableLcPtr :: new (BN_new ()) ? ; if 1 != BN_set_u64 (* bn , value) { return Err (()) ; } Ok (bn) } } }
};
}
