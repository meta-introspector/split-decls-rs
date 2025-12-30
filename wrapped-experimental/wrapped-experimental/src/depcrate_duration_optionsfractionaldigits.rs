// Generated macro for FractionalDigits (enum)
macro_rules! Depcrate_duration_optionsFractionalDigits {
() => {
// Module: crate::duration::options
// Provides: {"FractionalDigits"}
// Dependencies: {}
# [doc = " Options for configuring the number of fractional digits to display."] # [derive (Debug , Default , Clone , Copy , PartialEq , Eq)] pub enum FractionalDigits { # [doc = " Show as many fractional digits as necessary to display the whole duration,"] # [doc = " omitting trailing zeroes after the decimal point."] # [default] ShowAll , # [doc = " Use the given number of fractional digits."] # [doc = " This value must be in the range 0..=9."] # [doc = " Fractional digits are truncated if necessary."] Fixed (u8) , }
};
}
