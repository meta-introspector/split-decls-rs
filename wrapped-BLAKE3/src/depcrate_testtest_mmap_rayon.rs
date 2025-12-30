// Generated macro for test_mmap_rayon (function)
macro_rules! Depcrate_testtest_mmap_rayon {
() => {
// Module: crate::test
// Provides: {"test_mmap_rayon"}
// Dependencies: {}
# [test] # [cfg (feature = "mmap")] # [cfg (feature = "rayon")] # [cfg (not (miri))] fn test_mmap_rayon () -> Result < () , std :: io :: Error > { use std :: io :: prelude :: * ; let mut input = vec ! [0 ; 1_000_000] ; paint_test_input (& mut input) ; let mut tempfile = tempfile :: NamedTempFile :: new () ? ; tempfile . write_all (& input) ? ; tempfile . flush () ? ; assert_eq ! (crate :: Hasher :: new () . update_mmap_rayon (tempfile . path ()) ? . finalize () , crate :: hash (& input) ,) ; Ok (()) }
};
}
