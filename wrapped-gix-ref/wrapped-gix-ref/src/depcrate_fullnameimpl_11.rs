// Generated macro for impl_11 (impl)
macro_rules! Depcrate_fullnameimpl_11 {
() => {
// Module: crate::fullname
// Provides: {"impl_11"}
// Dependencies: {}
impl TryFrom < String > for FullName { type Error = gix_validate :: reference :: name :: Error ; fn try_from (value : String) -> Result < Self , Self :: Error > { gix_validate :: reference :: name (value . as_bytes () . as_bstr ()) ? ; Ok (FullName (value . into ())) } }
};
}
