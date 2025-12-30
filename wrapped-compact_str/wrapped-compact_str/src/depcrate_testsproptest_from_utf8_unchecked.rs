// Generated macro for proptest_from_utf8_unchecked (function)
macro_rules! Depcrate_testsproptest_from_utf8_unchecked {
() => {
// Module: crate::tests
// Provides: {"proptest_from_utf8_unchecked"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_from_utf8_unchecked (# [strategy (rand_unicode ())] std_str : String) { let bytes = std_str . as_bytes () ; let compact = unsafe { CompactString :: from_utf8_unchecked (bytes) } ; assert_eq ! (compact . as_bytes () , std_str . as_bytes ()) ; assert_eq ! (compact . as_bytes () , bytes) ; assert_eq ! (compact . len () , bytes . len ()) ; let data_is_valid = core :: str :: from_utf8 (bytes) ; let compact_is_valid = core :: str :: from_utf8 (compact . as_bytes ()) ; let std_str_is_valid = core :: str :: from_utf8 (std_str . as_bytes ()) ; match (data_is_valid , compact_is_valid , std_str_is_valid) { (Ok (d) , Ok (c) , Ok (s)) => { assert_eq ! (d , c) ; assert_eq ! (c , s) ; } (Err (d) , Err (c) , Err (s)) => { assert_eq ! (d , c) ; assert_eq ! (c , s) ; } _ => panic ! ("data, CompactString, and String disagreed?") , } }
};
}
