// Generated macro for impl_42 (impl)
macro_rules! Depcrate_nameimpl_42 {
() => {
// Module: crate::name
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a > convert :: TryFrom < & 'a BString > for & 'a PartialNameRef { type Error = Error ; fn try_from (v : & 'a BString) -> Result < Self , Self :: Error > { Ok (PartialNameRef :: new_unchecked (gix_validate :: reference :: name_partial (v . as_ref () ,) ?)) } }
};
}
