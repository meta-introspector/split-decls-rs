// Generated macro for impl_52 (impl)
macro_rules! Depcrate_nameimpl_52 {
() => {
// Module: crate::name
// Provides: {"impl_52"}
// Dependencies: {}
impl convert :: TryFrom < BString > for PartialName { type Error = Error ; fn try_from (v : BString) -> Result < Self , Self :: Error > { gix_validate :: reference :: name_partial (v . as_ref ()) ? ; Ok (PartialName (v)) } }
};
}
