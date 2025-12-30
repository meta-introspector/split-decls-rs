// Generated macro for impl_50 (impl)
macro_rules! Depcrate_nameimpl_50 {
() => {
// Module: crate::name
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'a > convert :: TryFrom < & 'a String > for & 'a PartialNameRef { type Error = Error ; fn try_from (v : & 'a String) -> Result < Self , Self :: Error > { let v = v . as_bytes () . as_bstr () ; Ok (PartialNameRef :: new_unchecked (gix_validate :: reference :: name_partial (v) ?)) } }
};
}
