// Generated macro for impl_12 (impl)
macro_rules! Depcrate_fullnameimpl_12 {
() => {
// Module: crate::fullname
// Provides: {"impl_12"}
// Dependencies: {}
impl TryFrom < & BStr > for FullName { type Error = gix_validate :: reference :: name :: Error ; fn try_from (value : & BStr) -> Result < Self , Self :: Error > { Ok (FullName (gix_validate :: reference :: name (value) ? . into ())) } }
};
}
