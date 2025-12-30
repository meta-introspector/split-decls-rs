// Generated macro for impl_20 (impl)
macro_rules! Depcrate_fieldsimpl_20 {
() => {
// Module: crate::fields
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a > Fields < 'a > { # [doc = " Create a new field iterator from an MCF hash, returning an error in the event the hash"] # [doc = " doesn't start with a leading `$` prefix."] # [doc = ""] # [doc = " NOTE: this method is deliberately non-public because it doesn't first validate the fields"] # [doc = " are well-formed. Calling it with non-validated inputs can lead to invalid [`Field`] values."] pub (crate) fn new (s : & 'a str) -> Self { let mut ret = Self (s) ; let should_be_empty = ret . next () . expect ("shouldn't be empty") ; debug_assert_eq ! (should_be_empty . as_str () , "") ; ret } }
};
}
