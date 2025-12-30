// Generated macro for EncodeStep (enum)
macro_rules! Depcrate_alphabetEncodeStep {
() => {
// Module: crate::alphabet
// Provides: {"EncodeStep"}
// Dependencies: {}
# [doc = " Constant-time encoder step."] # [derive (Copy , Clone , Debug)] pub enum EncodeStep { # [doc = " Apply the given offset to the cumulative result on match."] Apply (u8 , i16) , # [doc = " Compute a difference using the given offset on match."] Diff (u8 , i16) , }
};
}
