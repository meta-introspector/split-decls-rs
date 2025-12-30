// Generated macro for FormatParamUsage (enum)
macro_rules! Depcrate_macrosFormatParamUsage {
() => {
// Module: crate::macros
// Provides: {"FormatParamUsage"}
// Dependencies: {}
# [doc = " Where a format parameter is being used in the format string"] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum FormatParamUsage { # [doc = " Appears as an argument, e.g. `format!(\"{}\", foo)`"] Argument , # [doc = " Appears as a width, e.g. `format!(\"{:width$}\", foo, width = 1)`"] Width , # [doc = " Appears as a precision, e.g. `format!(\"{:.precision$}\", foo, precision = 1)`"] Precision , }
};
}
