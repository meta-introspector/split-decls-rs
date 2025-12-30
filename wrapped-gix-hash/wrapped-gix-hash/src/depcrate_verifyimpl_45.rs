// Generated macro for impl_45 (impl)
macro_rules! Depcrate_verifyimpl_45 {
() => {
// Module: crate::verify
// Provides: {"impl_45"}
// Dependencies: {}
impl oid { # [doc = " Verify that `self` matches the `expected` object ID."] # [doc = ""] # [doc = " Returns an [`Error`] containing both object IDs if they differ."] # [inline] pub fn verify (& self , expected : & oid) -> Result < () , Error > { if self == expected { Ok (()) } else { Err (Error { actual : self . to_owned () , expected : expected . to_owned () , }) } } }
};
}
