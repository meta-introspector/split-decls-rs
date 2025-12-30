// Generated macro for WeekdayNameLength (enum)
macro_rules! Depcrate_pattern_namesWeekdayNameLength {
() => {
// Module: crate::pattern::names
// Provides: {"WeekdayNameLength"}
// Dependencies: {}
# [doc = " Choices for loading weekday names."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [non_exhaustive] pub enum WeekdayNameLength { # [doc = " An abbreviated weekday name for formatting with other fields."] # [doc = ""] # [doc = " Example: \"Tue\""] Abbreviated , # [doc = " A wide weekday name for formatting with other fields."] # [doc = ""] # [doc = " Example: \"Tuesday\""] Wide , # [doc = " A narrow weekday name for formatting with other fields. Not necesarily unique."] # [doc = ""] # [doc = " Example: \"T\""] Narrow , # [doc = " A short weekday name for formatting with other fields."] # [doc = ""] # [doc = " Example: \"Tu\""] Short , # [doc = " An abbreviated weekday name for stand-alone display."] # [doc = ""] # [doc = " Example: \"Tue\""] StandaloneAbbreviated , # [doc = " A wide weekday name for stand-alone display."] # [doc = ""] # [doc = " Example: \"Tuesday\""] StandaloneWide , # [doc = " A narrow weekday name for stand-alone display. Not necesarily unique."] # [doc = ""] # [doc = " Example: \"T\""] StandaloneNarrow , # [doc = " A short weekday name for stand-alone display."] # [doc = ""] # [doc = " Example: \"Tu\""] StandaloneShort , }
};
}
