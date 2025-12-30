// Generated macro for impl_47 (impl)
macro_rules! Depcrate_nameimpl_47 {
() => {
// Module: crate::name
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a > convert :: TryFrom < & 'a str > for PartialName { type Error = Error ; fn try_from (v : & 'a str) -> Result < Self , Self :: Error > { let v = v . as_bytes () . as_bstr () ; Ok (PartialName (gix_validate :: reference :: name_partial (v) ? . to_owned ())) } }
};
}
