// Generated macro for impl_177 (impl)
macro_rules! Depcrateimpl_177 {
() => {
// Module: crate
// Provides: {"impl_177"}
// Dependencies: {}
impl TrustAnchor < '_ > { # [doc = " Yield a `'static` lifetime of the `TrustAnchor` by allocating owned `Der` variants"] # [cfg (feature = "alloc")] pub fn to_owned (& self) -> TrustAnchor < 'static > { # [cfg (not (feature = "std"))] use alloc :: borrow :: ToOwned ; TrustAnchor { subject : self . subject . as_ref () . to_owned () . into () , subject_public_key_info : self . subject_public_key_info . as_ref () . to_owned () . into () , name_constraints : self . name_constraints . as_ref () . map (| nc | nc . as_ref () . to_owned () . into ()) , } } }
};
}
