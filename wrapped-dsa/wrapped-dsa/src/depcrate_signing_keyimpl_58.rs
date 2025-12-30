// Generated macro for impl_58 (impl)
macro_rules! Depcrate_signing_keyimpl_58 {
() => {
// Module: crate::signing_key
// Provides: {"impl_58"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] impl EncodePrivateKey for SigningKey { fn to_pkcs8_der (& self) -> pkcs8 :: Result < SecretDocument > { let parameters = self . verifying_key () . components () . to_der () ? ; let parameters = AnyRef :: from_der (& parameters) ? ; let algorithm = AlgorithmIdentifierRef { oid : OID , parameters : Some (parameters) , } ; let mut x_bytes = self . x () . to_be_bytes () ; let x = UintRef :: new (& x_bytes) ? ; let mut signing_key = x . to_der () ? ; let signing_key_info = PrivateKeyInfoRef :: new (algorithm , OctetStringRef :: new (& signing_key) ?) ; let secret_document = signing_key_info . try_into () ? ; signing_key . zeroize () ; x_bytes . zeroize () ; Ok (secret_document) } }
};
}
