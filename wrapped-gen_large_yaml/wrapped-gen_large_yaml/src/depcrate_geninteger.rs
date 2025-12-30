// Generated macro for integer (function)
macro_rules! Depcrate_geninteger {
() => {
// Module: crate::gen
// Provides: {"integer"}
// Dependencies: {}
# [doc = " Generate a random integer."] pub fn integer (rng : & mut SmallRng , lo : i64 , hi : i64) -> i64 { rng . gen_range (lo .. hi) }
};
}
