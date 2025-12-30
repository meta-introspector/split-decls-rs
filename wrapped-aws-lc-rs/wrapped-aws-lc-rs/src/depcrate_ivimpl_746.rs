// Generated macro for impl_746 (impl)
macro_rules! Depcrate_ivimpl_746 {
() => {
// Module: crate::iv
// Provides: {"impl_746"}
// Dependencies: {}
impl < const L : usize > TryFrom < & [u8] > for FixedLength < L > { type Error = Unspecified ; fn try_from (value : & [u8]) -> Result < Self , Self :: Error > { let value : & [u8 ; L] = value . try_into () ? ; Ok (Self :: from (* value)) } }
};
}
