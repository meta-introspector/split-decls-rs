// Generated macro for VerifyingKey (struct)
macro_rules! Depcrate_ots_publicVerifyingKey {
() => {
// Module: crate::ots::public
// Provides: {"VerifyingKey"}
// Dependencies: {}
# [derive (Debug)] # [doc = " Opaque struct representing a LM-OTS public key"] pub struct VerifyingKey < Mode : LmsOtsMode > { pub (crate) q : u32 , pub (crate) id : Identifier , pub (crate) k : Output < Mode :: Hasher > , }
};
}
