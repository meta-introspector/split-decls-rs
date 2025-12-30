// Generated macro for impl_48 (impl)
macro_rules! Depcrate_nameimpl_48 {
() => {
// Module: crate::name
// Provides: {"impl_48"}
// Dependencies: {}
# [allow (clippy :: infallible_try_from)] impl < 'a > convert :: TryFrom < & 'a FullName > for & 'a PartialNameRef { type Error = Infallible ; fn try_from (v : & 'a FullName) -> Result < Self , Self :: Error > { Ok (v . as_ref () . as_partial_name ()) } }
};
}
