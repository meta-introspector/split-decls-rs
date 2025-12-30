// Generated macro for impl_747 (impl)
macro_rules! Depcrate_ivimpl_747 {
() => {
// Module: crate::iv
// Provides: {"impl_747"}
// Dependencies: {}
impl < const L : usize > TryFrom < FixedLength < L > > for [u8 ; L] { type Error = Unspecified ; fn try_from (value : FixedLength < L >) -> Result < Self , Self :: Error > { Ok (value . 0) } }
};
}
