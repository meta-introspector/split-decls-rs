// Generated macro for ValidatePublicKey (trait)
macro_rules! Depcrate_sec1ValidatePublicKey {
() => {
// Module: crate::sec1
// Provides: {"ValidatePublicKey"}
// Dependencies: {}
# [doc = " Validate that the given [`EncodedPoint`] represents the encoded public key"] # [doc = " value of the given secret."] # [doc = ""] # [doc = " Curve implementations which also impl [`CurveArithmetic`] will receive"] # [doc = " a blanket default impl of this trait."] pub trait ValidatePublicKey where Self : Curve , FieldBytesSize < Self > : ModulusSize , { # [doc = " Validate that the given [`EncodedPoint`] is a valid public key for the"] # [doc = " provided secret value."] # [allow (unused_variables)] fn validate_public_key (secret_key : & SecretKey < Self > , public_key : & EncodedPoint < Self > ,) -> Result < () > { Ok (()) } }
};
}
