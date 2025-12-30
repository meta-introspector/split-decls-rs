// Generated macro for TimeType (enum)
macro_rules! Depcrate_output_tableTimeType {
() => {
// Module: crate::output::table
// Provides: {"TimeType"}
// Dependencies: {}
# [doc = " The types of a file’s time fields. These three fields are standard"] # [doc = " across most (all?) operating systems."] # [derive (PartialEq , Eq , Debug , Copy , Clone)] pub enum TimeType { # [doc = " The file’s modified time (`st_mtime`)."] Modified , # [doc = " The file’s changed time (`st_ctime`)"] Changed , # [doc = " The file’s accessed time (`st_atime`)."] Accessed , # [doc = " The file’s creation time (`btime` or `birthtime`)."] Created , }
};
}
