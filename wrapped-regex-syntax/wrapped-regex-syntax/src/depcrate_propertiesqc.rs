// Generated macro for qc (function)
macro_rules! Depcrate_propertiesqc {
() => {
// Module: crate::properties
// Provides: {"qc"}
// Dependencies: {}
fn qc < T : Testable > (t : T) { QuickCheck :: new () . tests (10_000) . max_tests (20_000) . quickcheck (t) ; }
};
}
