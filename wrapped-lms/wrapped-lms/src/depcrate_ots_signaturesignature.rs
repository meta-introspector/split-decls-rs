// Generated macro for Signature (struct)
macro_rules! Depcrate_ots_signatureSignature {
() => {
// Module: crate::ots::signature
// Provides: {"Signature"}
// Dependencies: {}
# [derive (Debug , Eq)] # [doc = " Opaque struct representing a LM-OTS signature"] pub struct Signature < Mode : LmsOtsMode > { pub (crate) c : digest :: Output < Mode :: Hasher > , pub (crate) y : Array < digest :: Output < Mode :: Hasher > , Mode :: PLen > , }
};
}
