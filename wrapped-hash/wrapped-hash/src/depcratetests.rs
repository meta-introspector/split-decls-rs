// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_new_unique () { assert ! (Hash :: new_unique () != Hash :: new_unique ()) ; } # [test] fn test_hash_fromstr () { let hash = Hash :: new_from_array ([1 ; 32]) ; let mut hash_base58_str = bs58 :: encode (hash) . into_string () ; assert_eq ! (hash_base58_str . parse ::< Hash > () , Ok (hash)) ; hash_base58_str . push_str (& bs58 :: encode (hash . as_ref ()) . into_string ()) ; assert_eq ! (hash_base58_str . parse ::< Hash > () , Err (ParseHashError :: WrongSize)) ; hash_base58_str . truncate (hash_base58_str . len () / 2) ; assert_eq ! (hash_base58_str . parse ::< Hash > () , Ok (hash)) ; hash_base58_str . truncate (hash_base58_str . len () / 2) ; assert_eq ! (hash_base58_str . parse ::< Hash > () , Err (ParseHashError :: WrongSize)) ; let input_too_big = bs58 :: encode (& [0xffu8 ; HASH_BYTES + 1]) . into_string () ; assert ! (input_too_big . len () > MAX_BASE58_LEN) ; assert_eq ! (input_too_big . parse ::< Hash > () , Err (ParseHashError :: WrongSize)) ; let mut hash_base58_str = bs58 :: encode (hash . as_ref ()) . into_string () ; assert_eq ! (hash_base58_str . parse ::< Hash > () , Ok (hash)) ; hash_base58_str . replace_range (.. 1 , "I") ; assert_eq ! (hash_base58_str . parse ::< Hash > () , Err (ParseHashError :: Invalid)) ; } }
};
}
