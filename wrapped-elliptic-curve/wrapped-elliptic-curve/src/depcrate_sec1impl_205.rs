// Generated macro for impl_205 (impl)
macro_rules! Depcrate_sec1impl_205 {
() => {
// Module: crate::sec1
// Provides: {"impl_205"}
// Dependencies: {}
# [cfg (feature = "arithmetic")] impl < C > ValidatePublicKey for C where C : CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { fn validate_public_key (secret_key : & SecretKey < C > , public_key : & EncodedPoint < C >) -> Result < () > { let pk = secret_key . public_key () . to_encoded_point (public_key . is_compressed ()) ; if public_key == & pk { Ok (()) } else { Err (Error) } } }
};
}
