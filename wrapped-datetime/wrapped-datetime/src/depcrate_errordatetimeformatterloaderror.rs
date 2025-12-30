// Generated macro for DateTimeFormatterLoadError (enum)
macro_rules! Depcrate_errorDateTimeFormatterLoadError {
() => {
// Module: crate::error
// Provides: {"DateTimeFormatterLoadError"}
// Dependencies: {}
# [doc = " An error from constructing a formatter."] # [derive (Display , Debug , Copy , Clone , PartialEq)] # [non_exhaustive] pub enum DateTimeFormatterLoadError { # [doc = " An error while loading display names for a field."] # [displaydoc ("{0}")] Names (PatternLoadError) , # [doc = " An error while loading some other required data,"] # [doc = " such as skeleton patterns or calendar conversions."] # [displaydoc ("{0}")] Data (DataError) , }
};
}
