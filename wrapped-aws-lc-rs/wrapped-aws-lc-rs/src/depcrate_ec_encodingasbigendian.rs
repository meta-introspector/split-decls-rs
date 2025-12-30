// Generated macro for AsBigEndian (trait)
macro_rules! Depcrate_ec_encodingAsBigEndian {
() => {
// Module: crate::ec::encoding
// Provides: {"AsBigEndian"}
// Dependencies: {}
# [doc = " Trait for values that can be serialized into a big-endian format"] pub trait AsBigEndian < T > { # [doc = " Serializes into a big-endian format."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns Unspecified if serialization fails."] fn as_be_bytes (& self) -> Result < T , crate :: error :: Unspecified > ; }
};
}
