// Generated macro for impl_43 (impl)
macro_rules! Depcrate_nameimpl_43 {
() => {
// Module: crate::name
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'a > convert :: TryFrom < & 'a BStr > for & 'a PartialNameRef { type Error = Error ; fn try_from (v : & 'a BStr) -> Result < Self , Self :: Error > { Ok (PartialNameRef :: new_unchecked (gix_validate :: reference :: name_partial (v) ?)) } }
};
}
