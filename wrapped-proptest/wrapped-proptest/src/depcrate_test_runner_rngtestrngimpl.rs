// Generated macro for TestRngImpl (enum)
macro_rules! Depcrate_test_runner_rngTestRngImpl {
() => {
// Module: crate::test_runner::rng
// Provides: {"TestRngImpl"}
// Dependencies: {}
# [derive (Clone , Debug)] enum TestRngImpl { XorShift (XorShiftRng) , ChaCha (ChaChaRng) , PassThrough { off : usize , end : usize , data : Arc < [u8] > , } , Recorder { rng : ChaChaRng , record : Vec < u8 > , } , }
};
}
