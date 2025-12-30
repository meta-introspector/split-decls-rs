// Generated macro for AsRawBytes (trait)
macro_rules! Depcrate_ec_encodingAsRawBytes {
() => {
// Module: crate::ec::encoding
// Provides: {"AsRawBytes"}
// Dependencies: {}
# [doc = " Trait for values that can be serialized into a raw format"] pub trait AsRawBytes < T > { # [doc = " Serializes into a raw format."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns Unspecified if serialization fails."] fn as_raw_bytes (& self) -> Result < T , crate :: error :: Unspecified > ; }
};
}
