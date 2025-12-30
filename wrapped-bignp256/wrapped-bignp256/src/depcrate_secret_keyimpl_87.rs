// Generated macro for impl_87 (impl)
macro_rules! Depcrate_secret_keyimpl_87 {
() => {
// Module: crate::secret_key
// Provides: {"impl_87"}
// Dependencies: {}
impl EncodePrivateKey for SecretKey { fn to_pkcs8_der (& self) -> pkcs8 :: Result < SecretDocument > { let algorithm_identifier = pkcs8 :: AlgorithmIdentifierRef { oid : ALGORITHM_OID , parameters : Some ((& BignP256 :: OID) . into ()) , } ; let ec_private_key = self . to_bytes () ; let pkcs8_key = pkcs8 :: PrivateKeyInfoRef :: new (algorithm_identifier , OctetStringRef :: new (& ec_private_key) ? ,) ; Ok (SecretDocument :: encode_msg (& pkcs8_key) ?) } }
};
}
