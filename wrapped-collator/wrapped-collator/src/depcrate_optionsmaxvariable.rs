// Generated macro for MaxVariable (enum)
macro_rules! Depcrate_optionsMaxVariable {
() => {
// Module: crate::options
// Provides: {"MaxVariable"}
// Dependencies: {}
# [doc = " What characters get shifted to the quaternary level"] # [doc = " with `AlternateHandling::Shifted`."] # [derive (Eq , PartialEq , Debug , Copy , Clone)] # [repr (u8)] # [non_exhaustive] pub enum MaxVariable { # [doc = " Characters classified as spaces are shifted."] Space = 0 , # [doc = " Characters classified as spaces or punctuation"] # [doc = " are shifted."] Punctuation = 1 , # [doc = " Characters classified as spaces, punctuation,"] # [doc = " or symbols are shifted."] Symbol = 2 , # [doc = " Characters classified as spaces, punctuation,"] # [doc = " symbols, or currency symbols are shifted."] Currency = 3 , }
};
}
