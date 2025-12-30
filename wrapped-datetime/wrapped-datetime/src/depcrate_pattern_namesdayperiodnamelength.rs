// Generated macro for DayPeriodNameLength (enum)
macro_rules! Depcrate_pattern_namesDayPeriodNameLength {
() => {
// Module: crate::pattern::names
// Provides: {"DayPeriodNameLength"}
// Dependencies: {}
# [doc = " Choices for loading day period names."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [non_exhaustive] pub enum DayPeriodNameLength { # [doc = " An abbreviated 12-hour day period name, including display names for 0h and 12h."] # [doc = ""] # [doc = " Examples:"] # [doc = ""] # [doc = " - \"AM\""] # [doc = " - \"mid.\""] Abbreviated , # [doc = " A wide 12-hour day period name, including display names for 0h and 12h."] # [doc = ""] # [doc = " The wide form may be the same as the abbreviated form if the \"real\" long form"] # [doc = " (eg \"ante meridiem\") is not customarily used."] # [doc = ""] # [doc = " Examples:"] # [doc = ""] # [doc = " - \"AM\""] # [doc = " - \"mignight\""] Wide , # [doc = " An abbreviated 12-hour day period name, including display names for 0h and 12h."] # [doc = ""] # [doc = " The narrow form must be unique, unlike some other fields."] # [doc = ""] # [doc = " Examples:"] # [doc = ""] # [doc = " - \"AM\""] # [doc = " - \"md\""] Narrow , }
};
}
