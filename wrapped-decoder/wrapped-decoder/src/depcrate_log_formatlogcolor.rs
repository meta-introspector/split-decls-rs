// Generated macro for LogColor (enum)
macro_rules! Depcrate_log_formatLogColor {
() => {
// Module: crate::log::format
// Provides: {"LogColor"}
// Dependencies: {}
# [doc = " Coloring options for [LogSegment]s."] # [derive (Debug , PartialEq , Clone , Copy)] pub (super) enum LogColor { # [doc = " User-defined color."] # [doc = ""] # [doc = " Use a string that can be parsed by the FromStr implementation"] # [doc = " of [colored::Color]."] Color (colored :: Color) , # [doc = " Color matching the default color for the log level."] # [doc = " Use `\"severity\"` as a format parameter to use this option."] SeverityLevel , # [doc = " Color matching the default color for the log level,"] # [doc = " but only if the log level is WARN or ERROR."] # [doc = ""] # [doc = " Use `\"werror\"` as a format parameter to use this option."] WarnError , }
};
}
