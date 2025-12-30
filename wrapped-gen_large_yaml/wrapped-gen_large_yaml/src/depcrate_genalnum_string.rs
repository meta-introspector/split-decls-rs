// Generated macro for alnum_string (function)
macro_rules! Depcrate_genalnum_string {
() => {
// Module: crate::gen
// Provides: {"alnum_string"}
// Dependencies: {}
# [doc = " Generate an alphanumeric string with a length between `lo_len` and `hi_len`."] pub fn alnum_string (rng : & mut SmallRng , lo_len : usize , hi_len : usize) -> String { let len = rng . gen_range (lo_len .. hi_len) ; rng . sample_iter (& Alphanumeric) . take (len) . map (char :: from) . collect () }
};
}
