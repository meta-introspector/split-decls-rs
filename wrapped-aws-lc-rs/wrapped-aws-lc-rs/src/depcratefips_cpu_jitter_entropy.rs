// Generated macro for fips_cpu_jitter_entropy (function)
macro_rules! Depcratefips_cpu_jitter_entropy {
() => {
// Module: crate
// Provides: {"fips_cpu_jitter_entropy"}
// Dependencies: {}
# [cfg (feature = "fips")] # [doc = " Panics if the underlying implementation is not using CPU jitter entropy, otherwise it returns."] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if the underlying implementation is not using CPU jitter entropy."] pub fn fips_cpu_jitter_entropy () { try_fips_cpu_jitter_entropy () . unwrap () ; }
};
}
