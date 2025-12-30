// Generated macro for impl_46 (impl)
macro_rules! Depcrate_nameimpl_46 {
() => {
// Module: crate::name
// Provides: {"impl_46"}
// Dependencies: {}
impl < 'a > convert :: TryFrom < & 'a str > for & 'a PartialNameRef { type Error = Error ; fn try_from (v : & 'a str) -> Result < Self , Self :: Error > { let v = v . as_bytes () . as_bstr () ; Ok (PartialNameRef :: new_unchecked (gix_validate :: reference :: name_partial (v) ?)) } }
};
}
