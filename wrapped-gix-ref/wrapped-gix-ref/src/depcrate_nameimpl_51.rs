// Generated macro for impl_51 (impl)
macro_rules! Depcrate_nameimpl_51 {
() => {
// Module: crate::name
// Provides: {"impl_51"}
// Dependencies: {}
impl convert :: TryFrom < String > for PartialName { type Error = Error ; fn try_from (v : String) -> Result < Self , Self :: Error > { gix_validate :: reference :: name_partial (v . as_bytes () . as_bstr ()) ? ; Ok (PartialName (v . into ())) } }
};
}
