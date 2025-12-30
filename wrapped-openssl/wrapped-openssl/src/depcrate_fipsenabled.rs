// Generated macro for enabled (function)
macro_rules! Depcrate_fipsenabled {
() => {
// Module: crate::fips
// Provides: {"enabled"}
// Dependencies: {}
# [doc = " Determines if the library is running in the FIPS 140-2 mode of operation."] # [corresponds (FIPS_mode)] pub fn enabled () -> bool { unsafe { ffi :: FIPS_mode () != 0 } }
};
}
