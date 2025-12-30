// Generated macro for CovTerm (enum)
macro_rules! Depcrate_covfunCovTerm {
() => {
// Module: crate::covfun
// Provides: {"CovTerm"}
// Dependencies: {}
# [doc = " Enum that can hold a constant zero value, the ID of an physical coverage"] # [doc = " counter, or the ID (and operation) of a coverage-counter expression."] # [doc = ""] # [doc = " Terms are used as the operands of coverage-counter expressions, as the arms"] # [doc = " of branch mappings, and as the value of code/gap mappings."] # [derive (Clone , Copy , Debug)] pub (crate) enum CovTerm { Zero , Counter (u32) , Expression (u32 , Op) , }
};
}
