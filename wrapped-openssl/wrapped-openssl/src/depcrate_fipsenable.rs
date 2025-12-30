// Generated macro for enable (function)
macro_rules! Depcrate_fipsenable {
() => {
// Module: crate::fips
// Provides: {"enable"}
// Dependencies: {}
# [doc = " Moves the library into or out of the FIPS 140-2 mode of operation."] # [corresponds (FIPS_mode_set)] pub fn enable (enabled : bool) -> Result < () , ErrorStack > { ffi :: init () ; unsafe { cvt (ffi :: FIPS_mode_set (enabled as _)) . map (| _ | ()) } }
};
}
