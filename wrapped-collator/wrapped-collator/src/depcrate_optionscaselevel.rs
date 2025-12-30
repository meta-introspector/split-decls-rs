// Generated macro for CaseLevel (enum)
macro_rules! Depcrate_optionsCaseLevel {
() => {
// Module: crate::options
// Provides: {"CaseLevel"}
// Dependencies: {}
# [doc = " Whether to distinguish case in sorting, even for sorting levels higher"] # [doc = " than tertiary, without having to use tertiary level just to enable case level differences."] # [derive (Eq , PartialEq , Debug , Copy , Clone)] # [repr (u8)] # [non_exhaustive] pub enum CaseLevel { # [doc = " Leave off the case level option.  Case differences will be handled by default"] # [doc = " in tertiary strength."] Off = 0 , # [doc = " Turn on the case level option, thereby making a separate level for case"] # [doc = " differences, positioned between secondary and tertiary."] # [doc = ""] # [doc = " When used together with [`Strength::Primary`], this corresponds to the"] # [doc = " ECMA-402 sensitivity \"case\"."] On = 1 , }
};
}
