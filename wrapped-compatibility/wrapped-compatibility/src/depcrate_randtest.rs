// Generated macro for test (function)
macro_rules! Depcrate_randtest {
() => {
// Module: crate::rand
// Provides: {"test"}
// Dependencies: {}
# [test] pub fn test () { let mut rng = rand :: thread_rng () ; for _ in 0 .. 1000 { crate :: test_same (Lcg64Xsh32 { state : rng . gen () , increment : rng . gen () , }) ; } }
};
}
