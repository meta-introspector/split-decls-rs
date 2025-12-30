// Generated macro for Format (enum)
macro_rules! Depcrate_tableFormat {
() => {
// Module: crate::table
// Provides: {"Format"}
// Dependencies: {}
# [doc = " The format string to generate a time zone abbreviation from."] # [non_exhaustive] # [derive (PartialEq , Debug , Clone)] pub enum Format { # [doc = " A constant format, which remains the same throughout both standard"] # [doc = " and DST timespans."] Constant (String) , # [doc = " An alternate format, such as “PST/PDT”, which changes between"] # [doc = " standard and DST timespans."] Alternate { # [doc = " Abbreviation to use during Standard Time."] standard : String , # [doc = " Abbreviation to use during Summer Time."] dst : String , } , # [doc = " A format with a placeholder `%s`, which uses the `letters` field in"] # [doc = " a `RuleInfo` to generate the time zone abbreviation."] Placeholder (String) , # [doc = " The special %z placeholder that gets formatted as a numeric offset."] Offset , }
};
}
