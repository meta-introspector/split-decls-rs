// Generated macro for fips_mode (function)
macro_rules! Depcratefips_mode {
() => {
// Module: crate
// Provides: {"fips_mode"}
// Dependencies: {}
# [cfg (feature = "fips")] # [doc = " Panics if the underlying implementation is not FIPS, otherwise it returns."] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if the underlying implementation is not FIPS."] pub fn fips_mode () { try_fips_mode () . unwrap () ; }
};
}
