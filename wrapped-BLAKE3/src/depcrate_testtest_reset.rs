// Generated macro for test_reset (function)
macro_rules! Depcrate_testtest_reset {
() => {
// Module: crate::test
// Provides: {"test_reset"}
// Dependencies: {}
# [test] fn test_reset () { let mut hasher = crate :: Hasher :: new () ; hasher . update (& [42 ; 3 * CHUNK_LEN + 7]) ; hasher . reset () ; hasher . update (& [42 ; CHUNK_LEN + 3]) ; assert_eq ! (hasher . finalize () , crate :: hash (& [42 ; CHUNK_LEN + 3])) ; let key = & [99 ; crate :: KEY_LEN] ; let mut keyed_hasher = crate :: Hasher :: new_keyed (key) ; keyed_hasher . update (& [42 ; 3 * CHUNK_LEN + 7]) ; keyed_hasher . reset () ; keyed_hasher . update (& [42 ; CHUNK_LEN + 3]) ; assert_eq ! (keyed_hasher . finalize () , crate :: keyed_hash (key , & [42 ; CHUNK_LEN + 3]) ,) ; let context = "BLAKE3 2020-02-12 10:20:58 reset test" ; let mut kdf = crate :: Hasher :: new_derive_key (context) ; kdf . update (& [42 ; 3 * CHUNK_LEN + 7]) ; kdf . reset () ; kdf . update (& [42 ; CHUNK_LEN + 3]) ; let expected = crate :: derive_key (context , & [42 ; CHUNK_LEN + 3]) ; assert_eq ! (kdf . finalize () , expected) ; }
};
}
