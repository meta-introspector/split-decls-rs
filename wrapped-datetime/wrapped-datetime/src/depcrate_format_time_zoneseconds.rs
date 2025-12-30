// Generated macro for Seconds (enum)
macro_rules! Depcrate_format_time_zoneSeconds {
() => {
// Module: crate::format::time_zone
// Provides: {"Seconds"}
// Dependencies: {}
# [doc = " Whether the seconds field should be optional or excluded in ISO-8601 format."] # [derive (Debug , Clone , Copy , PartialEq)] enum Seconds { # [doc = " Seconds are displayed only if they are non-zero."] Optional , # [doc = " Seconds are not displayed."] Never , }
};
}
