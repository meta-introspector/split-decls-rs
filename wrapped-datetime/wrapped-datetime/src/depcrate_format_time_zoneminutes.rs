// Generated macro for Minutes (enum)
macro_rules! Depcrate_format_time_zoneMinutes {
() => {
// Module: crate::format::time_zone
// Provides: {"Minutes"}
// Dependencies: {}
# [doc = " Whether the minutes field should be optional or required in ISO-8601 format."] # [derive (Debug , Clone , Copy , PartialEq)] enum Minutes { # [doc = " Minutes are always displayed."] Required , # [doc = " Minutes are displayed only if they are non-zero."] Optional , }
};
}
