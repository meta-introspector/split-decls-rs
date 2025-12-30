// Generated macro for impl_14 (impl)
macro_rules! Depcrate_fullnameimpl_14 {
() => {
// Module: crate::fullname
// Provides: {"impl_14"}
// Dependencies: {}
impl TryFrom < & BString > for FullName { type Error = gix_validate :: reference :: name :: Error ; fn try_from (value : & BString) -> Result < Self , Self :: Error > { gix_validate :: reference :: name (value . as_ref ()) ? ; Ok (FullName (value . clone ())) } }
};
}
