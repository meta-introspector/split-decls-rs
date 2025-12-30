// Generated macro for impl_75 (impl)
macro_rules! Depcrate_lms_publicimpl_75 {
() => {
// Module: crate::lms::public
// Provides: {"impl_75"}
// Dependencies: {}
impl < Mode : LmsMode > Verifier < Signature < Mode > > for VerifyingKey < Mode > { fn verify (& self , msg : & [u8] , signature : & Signature < Mode >) -> Result < () , Error > { self . multipart_verify (& [msg] , signature) } }
};
}
