// Generated macro for impl_38 (impl)
macro_rules! Depcrate_nameimpl_38 {
() => {
// Module: crate::name
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'a > convert :: TryFrom < & 'a BStr > for & 'a FullNameRef { type Error = Error ; fn try_from (v : & 'a BStr) -> Result < Self , Self :: Error > { Ok (FullNameRef :: new_unchecked (gix_validate :: reference :: name (v) ?)) } }
};
}
