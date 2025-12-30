// Generated macro for try_fips_mode (function)
macro_rules! Depcratetry_fips_mode {
() => {
// Module: crate
// Provides: {"try_fips_mode"}
// Dependencies: {}
# [doc = " Indicates whether the underlying implementation is FIPS."] # [doc = ""] # [doc = " # Errors"] # [doc = " Return an error if the underlying implementation is not FIPS, otherwise Ok."] pub fn try_fips_mode () -> Result < () , & 'static str > { init () ; match unsafe { FIPS_mode () } { 1 => Ok (()) , _ => Err ("FIPS mode not enabled!") , } }
};
}
