// Generated macro for impl_135 (impl)
macro_rules! Depcrate_pkcs8impl_135 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_135"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < P > EncodePrivateKey for KeyPair < P > where P : MlDsaParams , P : AssociatedAlgorithmIdentifier < Params = AnyRef < 'static > > , { fn to_pkcs8_der (& self) -> :: pkcs8 :: Result < der :: SecretDocument > { let seed_der = SeedString { tag_mode : TagMode :: Implicit , tag_number : SEED_TAG_NUMBER , value : OctetStringRef :: new (& self . seed) ? , } . to_der () ? ; let private_key = OctetStringRef :: new (& seed_der) ? ; let private_key_info = PrivateKeyInfoRef :: new (P :: ALGORITHM_IDENTIFIER , private_key) ; :: pkcs8 :: SecretDocument :: encode_msg (& private_key_info) . map_err (:: pkcs8 :: Error :: Asn1) } }
};
}
