// Generated macro for impl_10 (impl)
macro_rules! Depcrate_fullnameimpl_10 {
() => {
// Module: crate::fullname
// Provides: {"impl_10"}
// Dependencies: {}
impl TryFrom < & str > for FullName { type Error = gix_validate :: reference :: name :: Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Ok (FullName (gix_validate :: reference :: name (value . as_bytes () . as_bstr ()) ? . into () ,)) } }
};
}
