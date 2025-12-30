// Generated macro for ListLength (enum)
macro_rules! Depcrate_optionsListLength {
() => {
// Module: crate::options
// Provides: {"ListLength"}
// Dependencies: {}
# [doc = " Represents the style of a list. See the"] # [doc = " [CLDR spec](https://unicode.org/reports/tr35/tr35-general.html#ListPatterns)"] # [doc = " for an explanation of the different styles."] # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash , Default)] # [non_exhaustive] pub enum ListLength { # [doc = " A typical list"] # [default] Wide , # [doc = " A shorter list"] Short , # [doc = " The shortest type of list"] Narrow , }
};
}
