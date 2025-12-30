// Generated macro for impl_90 (impl)
macro_rules! Depcrate_lms_signatureimpl_90 {
() => {
// Module: crate::lms::signature
// Provides: {"impl_90"}
// Dependencies: {}
impl < Mode : LmsMode > Clone for Signature < Mode > { fn clone (& self) -> Self { Self { q : self . q , lmots_sig : self . lmots_sig . clone () , path : self . path . clone () , } } }
};
}
