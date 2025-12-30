// Generated macro for YearAmbiguity (enum)
macro_rules! Depcrate_typesYearAmbiguity {
() => {
// Module: crate::types
// Provides: {"YearAmbiguity"}
// Dependencies: {}
# [doc = " Defines whether the era or century is required to interpret the year."] # [doc = ""] # [doc = " For example 2024 AD can be formatted as `2024`, or even `24`, but 1931 AD"] # [doc = " should not be formatted as `31`, and 2024 BC should not be formatted as `2024`."] # [derive (Copy , Clone , Debug , PartialEq)] # [allow (clippy :: exhaustive_enums)] pub enum YearAmbiguity { # [doc = " The year is unambiguous without a century or era."] Unambiguous , # [doc = " The century is required, the era may be included."] CenturyRequired , # [doc = " The era is required, the century may be included."] EraRequired , # [doc = " The century and era are required."] EraAndCenturyRequired , }
};
}
