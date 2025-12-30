// Generated macro for quickcheck (function)
macro_rules! Depcrate_testerquickcheck {
() => {
// Module: crate::tester
// Provides: {"quickcheck"}
// Dependencies: {}
# [doc = " Convenience function for running `QuickCheck`."] # [doc = ""] # [doc = " This is an alias for `QuickCheck::new().quickcheck(f)`."] pub fn quickcheck < A : Testable > (f : A) { QuickCheck :: new () . quickcheck (f) }
};
}
