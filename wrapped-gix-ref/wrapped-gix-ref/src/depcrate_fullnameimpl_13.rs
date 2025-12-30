// Generated macro for impl_13 (impl)
macro_rules! Depcrate_fullnameimpl_13 {
() => {
// Module: crate::fullname
// Provides: {"impl_13"}
// Dependencies: {}
impl TryFrom < BString > for FullName { type Error = gix_validate :: reference :: name :: Error ; fn try_from (value : BString) -> Result < Self , Self :: Error > { gix_validate :: reference :: name (value . as_ref ()) ? ; Ok (FullName (value)) } }
};
}
