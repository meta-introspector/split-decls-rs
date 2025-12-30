// Generated macro for test_reset (function)
macro_rules! Depcrate_testtest_reset {
() => {
// Module: crate::test
// Provides: {"test_reset"}
// Dependencies: {}
# [test] fn test_reset () { { let mut hasher = crate :: Hasher :: new () ; hasher . update (& [42 ; 3 * CHUNK_LEN + 7]) ; hasher . reset () ; hasher . update (& [42 ; CHUNK_LEN + 3]) ; let mut output = [0 ; 32] ; hasher . finalize (& mut output) ; let mut reference_hasher = reference_impl :: Hasher :: new () ; reference_hasher . update (& [42 ; CHUNK_LEN + 3]) ; let mut reference_hash = [0 ; 32] ; reference_hasher . finalize (& mut reference_hash) ; assert_eq ! (reference_hash , output) ; } { let key = & [99 ; 32] ; let mut hasher = crate :: Hasher :: new_keyed (key) ; hasher . update (& [42 ; 3 * CHUNK_LEN + 7]) ; hasher . reset () ; hasher . update (& [42 ; CHUNK_LEN + 3]) ; let mut output = [0 ; 32] ; hasher . finalize (& mut output) ; let mut reference_hasher = reference_impl :: Hasher :: new_keyed (key) ; reference_hasher . update (& [42 ; CHUNK_LEN + 3]) ; let mut reference_hash = [0 ; 32] ; reference_hasher . finalize (& mut reference_hash) ; assert_eq ! (reference_hash , output) ; } { let context = "BLAKE3 2020-02-12 10:20:58 reset test" ; let mut hasher = crate :: Hasher :: new_derive_key (context) ; hasher . update (& [42 ; 3 * CHUNK_LEN + 7]) ; hasher . reset () ; hasher . update (& [42 ; CHUNK_LEN + 3]) ; let mut output = [0 ; 32] ; hasher . finalize (& mut output) ; let mut reference_hasher = reference_impl :: Hasher :: new_derive_key (context) ; reference_hasher . update (& [42 ; CHUNK_LEN + 3]) ; let mut reference_hash = [0 ; 32] ; reference_hasher . finalize (& mut reference_hash) ; assert_eq ! (reference_hash , output) ; } }
};
}
