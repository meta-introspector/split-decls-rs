// Generated macro for impl_134 (impl)
macro_rules! Depcrate_pkcs8impl_134 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_134"}
// Dependencies: {}
impl < P > TryFrom < PrivateKeyInfoRef < '_ > > for KeyPair < P > where P : MlDsaParams , P : AssociatedAlgorithmIdentifier < Params = AnyRef < 'static > > , { type Error = :: pkcs8 :: Error ; fn try_from (private_key_info : PrivateKeyInfoRef < '_ >) -> :: pkcs8 :: Result < Self > { private_key_info . algorithm . assert_algorithm_oid (P :: ALGORITHM_IDENTIFIER . oid) ? ; let mut reader = der :: SliceReader :: new (private_key_info . private_key . as_bytes ()) ? ; let seed_string = SeedString :: decode_implicit (& mut reader , SEED_TAG_NUMBER) ? . ok_or (pkcs8 :: Error :: KeyMalformed) ? ; let seed = seed_string . value . as_bytes () . try_into () . map_err (| _ | pkcs8 :: Error :: KeyMalformed) ? ; reader . finish () ? ; Ok (P :: from_seed (& seed)) } }
};
}
