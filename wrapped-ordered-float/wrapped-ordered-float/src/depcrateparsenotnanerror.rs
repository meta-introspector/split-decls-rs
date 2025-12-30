// Generated macro for ParseNotNanError (enum)
macro_rules! DepcrateParseNotNanError {
() => {
// Module: crate
// Provides: {"ParseNotNanError"}
// Dependencies: {}
# [doc = " An error indicating a parse error from a string for `NotNan`."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum ParseNotNanError < E > { # [doc = " A plain parse error from the underlying float type."] ParseFloatError (E) , # [doc = " The parsed float value resulted in a NaN."] IsNaN , }
};
}
