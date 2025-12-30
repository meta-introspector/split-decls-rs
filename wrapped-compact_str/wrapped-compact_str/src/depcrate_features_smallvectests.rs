// Generated macro for tests (module)
macro_rules! Depcrate_features_smallvectests {
() => {
// Module: crate::features::smallvec
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use alloc :: string :: String ; use proptest :: prelude :: * ; use test_strategy :: proptest ; use crate :: repr :: MAX_SIZE ; use crate :: tests :: rand_unicode ; use crate :: CompactString ; # [doc = " generates random unicode strings, that are at least MAX_SIZE bytes long"] pub fn rand_long_unicode () -> impl Strategy < Value = String > { proptest :: collection :: vec (proptest :: char :: any () , (MAX_SIZE + 1) .. 80) . prop_map (| v | v . into_iter () . collect ()) } # [test] fn test_buffer_reuse () { let c = CompactString :: from ("I am a longer string that will be on the heap") ; let c_ptr = c . as_ptr () ; let bytes = c . into_bytes () ; let b_ptr = bytes . as_ptr () ; assert_eq ! (c_ptr , b_ptr) ; } # [proptest] # [cfg_attr (miri , ignore)] fn proptest_buffer_reuse (# [strategy (rand_long_unicode ())] s : String) { let c = CompactString :: from (s) ; let c_ptr = c . as_ptr () ; let bytes = c . into_bytes () ; let b_ptr = bytes . as_ptr () ; prop_assert_eq ! (c_ptr , b_ptr) ; } # [proptest] # [cfg_attr (miri , ignore)] fn proptest_roundtrip (# [strategy (rand_unicode ())] s : String) { let og_compact = CompactString :: from (s . clone ()) ; prop_assert_eq ! (& og_compact , & s) ; let bytes = og_compact . into_bytes () ; let ex_compact = CompactString :: from_utf8 (bytes) . unwrap () ; prop_assert_eq ! (& ex_compact , & s) ; } }
};
}
