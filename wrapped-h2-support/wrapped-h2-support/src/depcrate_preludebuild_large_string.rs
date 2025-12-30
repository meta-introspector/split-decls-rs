// Generated macro for build_large_string (function)
macro_rules! Depcrate_preludebuild_large_string {
() => {
// Module: crate::prelude
// Provides: {"build_large_string"}
// Dependencies: {}
fn build_large_string (ch : char , len : usize) -> String { let mut ret = String :: new () ; for _ in 0 .. len { ret . push (ch) ; } ret }
};
}
