// Generated macro for impl_49 (impl)
macro_rules! Depcrate_nameimpl_49 {
() => {
// Module: crate::name
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a > convert :: TryFrom < & 'a String > for & 'a FullNameRef { type Error = Error ; fn try_from (v : & 'a String) -> Result < Self , Self :: Error > { let v = v . as_bytes () . as_bstr () ; Ok (FullNameRef :: new_unchecked (gix_validate :: reference :: name (v) ?)) } }
};
}
