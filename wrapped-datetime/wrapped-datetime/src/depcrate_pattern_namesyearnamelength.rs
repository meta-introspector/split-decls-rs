// Generated macro for YearNameLength (enum)
macro_rules! Depcrate_pattern_namesYearNameLength {
() => {
// Module: crate::pattern::names
// Provides: {"YearNameLength"}
// Dependencies: {}
# [doc = " Choices for loading year names."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [non_exhaustive] pub enum YearNameLength { # [doc = " An abbreviated calendar-dependent year or era name."] # [doc = ""] # [doc = " Examples:"] # [doc = ""] # [doc = " - \"AD\""] # [doc = " - \"甲子\""] Abbreviated , # [doc = " A wide calendar-dependent year or era name."] # [doc = ""] # [doc = " Examples:"] # [doc = ""] # [doc = " - \"Anno Domini\""] # [doc = " - \"甲子\""] Wide , # [doc = " A narrow calendar-dependent year or era name. Not necesarily unique."] # [doc = ""] # [doc = " Examples:"] # [doc = ""] # [doc = " - \"A\""] # [doc = " - \"甲子\""] Narrow , }
};
}
