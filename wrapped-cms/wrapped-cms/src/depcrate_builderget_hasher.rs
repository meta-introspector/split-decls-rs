// Generated macro for get_hasher (function)
macro_rules! Depcrate_builderget_hasher {
() => {
// Module: crate::builder
// Provides: {"get_hasher"}
// Dependencies: {}
# [doc = " Get a hasher for a given digest algorithm"] fn get_hasher (digest_algorithm_identifier : & AlgorithmIdentifierOwned ,) -> Option < Box < dyn DynDigest > > { let digest_name = DB . by_oid (& digest_algorithm_identifier . oid) ? ; match digest_name { "id-sha1" => Some (Box :: new (sha1 :: Sha1 :: new ())) , "id-sha256" => Some (Box :: new (sha2 :: Sha256 :: new ())) , "id-sha384" => Some (Box :: new (sha2 :: Sha384 :: new ())) , "id-sha512" => Some (Box :: new (sha2 :: Sha512 :: new ())) , "id-sha224" => Some (Box :: new (sha2 :: Sha224 :: new ())) , "id-sha-3-224" => Some (Box :: new (sha3 :: Sha3_224 :: new ())) , "id-sha-3-256" => Some (Box :: new (sha3 :: Sha3_256 :: new ())) , "id-sha-3-384" => Some (Box :: new (sha3 :: Sha3_384 :: new ())) , "id-sha-3-512" => Some (Box :: new (sha3 :: Sha3_512 :: new ())) , _ => None , } }
};
}
