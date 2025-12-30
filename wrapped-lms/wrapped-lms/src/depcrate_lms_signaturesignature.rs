// Generated macro for Signature (struct)
macro_rules! Depcrate_lms_signatureSignature {
() => {
// Module: crate::lms::signature
// Provides: {"Signature"}
// Dependencies: {}
# [doc = " Opaque struct representing a LMS signature"] pub struct Signature < Mode : LmsMode > { pub (crate) q : u32 , pub (crate) lmots_sig : OtsSignature < Mode :: OtsMode > , pub (crate) path : Array < digest :: Output < Mode :: Hasher > , Mode :: HLen > , }
};
}
