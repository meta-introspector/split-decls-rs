// Generated macro for AsDer (trait)
macro_rules! Depcrate_ec_encodingAsDer {
() => {
// Module: crate::ec::encoding
// Provides: {"AsDer"}
// Dependencies: {}
# [doc = " Trait for types that can be serialized into a DER format."] pub trait AsDer < T > { # [doc = " Serializes into a DER format."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns Unspecified if serialization fails."] fn as_der (& self) -> Result < T , crate :: error :: Unspecified > ; }
};
}
