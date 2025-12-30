// Generated macro for impl_44 (impl)
macro_rules! Depcrate_nameimpl_44 {
() => {
// Module: crate::name
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'a > convert :: TryFrom < & 'a PartialName > for & 'a PartialNameRef { type Error = Error ; fn try_from (v : & 'a PartialName) -> Result < Self , Self :: Error > { Ok (PartialNameRef :: new_unchecked (v . 0 . as_bstr ())) } }
};
}
