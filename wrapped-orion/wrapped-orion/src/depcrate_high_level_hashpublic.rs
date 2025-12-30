// Generated macro for public (module)
macro_rules! Depcrate_high_level_hashpublic {
() => {
// Module: crate::high_level::hash
// Provides: {"public"}
// Dependencies: {}
# [cfg (feature = "safe_api")] # [cfg (test)] mod public { use super :: * ; # [quickcheck] # [doc = " Hashing twice with same input should always produce same output."] fn prop_digest_same_result (input : Vec < u8 >) -> bool { digest (& input [..]) . unwrap () == digest (& input [..]) . unwrap () } # [quickcheck] # [doc = " Hashing all input should be the same as wrapping it in a"] # [doc = " cursor and using digest_from_reader."] fn prop_digest_same_as_digest_from_reader (input : Vec < u8 >) -> bool { let digest_a = digest_from_reader (std :: io :: Cursor :: new (& input)) . unwrap () ; let digest_b = digest (& input) . unwrap () ; digest_a == digest_b } # [quickcheck] # [doc = " Hashing twice with different input should never produce same output."] fn prop_digest_diff_result (input : Vec < u8 >) -> bool { digest (& input [..]) . unwrap () != digest (b"Completely wrong input") . unwrap () } }
};
}
