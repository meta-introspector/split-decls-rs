// Generated macro for SigningKey (struct)
macro_rules! Depcrate_ots_privateSigningKey {
() => {
// Module: crate::ots::private
// Provides: {"SigningKey"}
// Dependencies: {}
# [derive (Debug)] # [doc = " Opaque struct representing an LM-OTS private key. Does not implement"] # [doc = " [Clone] because OTS keys are supposed to be one time use."] pub struct SigningKey < Mode : LmsOtsMode > { q : u32 , id : Identifier , x : Array < Output < Mode :: Hasher > , Mode :: PLen > , valid : bool , }
};
}
