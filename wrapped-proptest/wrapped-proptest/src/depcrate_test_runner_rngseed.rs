// Generated macro for Seed (enum)
macro_rules! Depcrate_test_runner_rngSeed {
() => {
// Module: crate::test_runner::rng
// Provides: {"Seed"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord)] pub (crate) enum Seed { XorShift ([u8 ; 16]) , ChaCha ([u8 ; 32]) , PassThrough (Option < (usize , usize) > , Arc < [u8] >) , Recorder ([u8 ; 32]) , }
};
}
