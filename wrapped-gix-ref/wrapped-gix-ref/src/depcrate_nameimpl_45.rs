// Generated macro for impl_45 (impl)
macro_rules! Depcrate_nameimpl_45 {
() => {
// Module: crate::name
// Provides: {"impl_45"}
// Dependencies: {}
impl < 'a > convert :: TryFrom < & 'a str > for & 'a FullNameRef { type Error = Error ; fn try_from (v : & 'a str) -> Result < Self , Self :: Error > { let v = v . as_bytes () . as_bstr () ; Ok (FullNameRef :: new_unchecked (gix_validate :: reference :: name (v) ?)) } }
};
}
