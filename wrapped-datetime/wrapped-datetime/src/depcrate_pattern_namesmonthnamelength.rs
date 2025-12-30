// Generated macro for MonthNameLength (enum)
macro_rules! Depcrate_pattern_namesMonthNameLength {
() => {
// Module: crate::pattern::names
// Provides: {"MonthNameLength"}
// Dependencies: {}
# [doc = " Choices for loading month names."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [non_exhaustive] pub enum MonthNameLength { # [doc = " An abbreviated calendar-dependent month name for formatting with other fields."] # [doc = ""] # [doc = " Example: \"Sep\""] Abbreviated , # [doc = " A wide calendar-dependent month name for formatting with other fields."] # [doc = ""] # [doc = " Example: \"September\""] Wide , # [doc = " A narrow calendar-dependent month name for formatting with other fields. Not necesarily unique."] # [doc = ""] # [doc = " Example: \"S\""] Narrow , # [doc = " An abbreviated calendar-dependent month name for stand-alone display."] # [doc = ""] # [doc = " Example: \"Sep\""] StandaloneAbbreviated , # [doc = " A wide calendar-dependent month name for stand-alone display."] # [doc = ""] # [doc = " Example: \"September\""] StandaloneWide , # [doc = " A narrow calendar-dependent month name for stand-alone display. Not necesarily unique."] # [doc = ""] # [doc = " Example: \"S\""] StandaloneNarrow , }
};
}
