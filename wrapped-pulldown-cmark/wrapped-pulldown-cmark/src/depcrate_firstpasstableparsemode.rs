// Generated macro for TableParseMode (enum)
macro_rules! Depcrate_firstpassTableParseMode {
() => {
// Module: crate::firstpass
// Provides: {"TableParseMode"}
// Dependencies: {}
# [doc = " Scanning modes for `Parser`'s `parse_line` method."] # [derive (PartialEq , Eq , Copy , Clone)] enum TableParseMode { # [doc = " Inside a paragraph, scanning for table headers."] Scan , # [doc = " Inside a table."] Active , # [doc = " Inside a paragraph, not scanning for table headers."] Disabled , }
};
}
