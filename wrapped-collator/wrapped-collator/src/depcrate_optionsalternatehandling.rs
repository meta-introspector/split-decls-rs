// Generated macro for AlternateHandling (enum)
macro_rules! Depcrate_optionsAlternateHandling {
() => {
// Module: crate::options
// Provides: {"AlternateHandling"}
// Dependencies: {}
# [doc = " What to do about characters whose comparison level can be"] # [doc = " varied dynamically."] # [derive (Eq , PartialEq , Debug , Copy , Clone , PartialOrd , Ord)] # [repr (u8)] # [non_exhaustive] pub enum AlternateHandling { # [doc = " Keep the characters whose level can be varied on the"] # [doc = " primary level."] NonIgnorable = 0 , # [doc = " Shift the characters at or below `MaxVariable` to the"] # [doc = " quaternary level."] Shifted = 1 , }
};
}
