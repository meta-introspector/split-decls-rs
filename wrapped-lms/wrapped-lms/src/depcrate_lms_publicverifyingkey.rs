// Generated macro for VerifyingKey (struct)
macro_rules! Depcrate_lms_publicVerifyingKey {
() => {
// Module: crate::lms::public
// Provides: {"VerifyingKey"}
// Dependencies: {}
# [derive (Debug)] # [doc = " Opaque struct representing a LMS public key"] pub struct VerifyingKey < Mode : LmsMode > { pub (crate) id : Identifier , pub (crate) k : Output < Mode :: Hasher > , }
};
}
