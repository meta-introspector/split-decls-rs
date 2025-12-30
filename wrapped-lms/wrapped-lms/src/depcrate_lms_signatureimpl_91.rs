// Generated macro for impl_91 (impl)
macro_rules! Depcrate_lms_signatureimpl_91 {
() => {
// Module: crate::lms::signature
// Provides: {"impl_91"}
// Dependencies: {}
impl < Mode : LmsMode > PartialEq for Signature < Mode > { fn eq (& self , other : & Self) -> bool { self . q == other . q && self . lmots_sig == other . lmots_sig && self . path == other . path } }
};
}
