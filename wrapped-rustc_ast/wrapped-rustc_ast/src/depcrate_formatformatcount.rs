// Generated macro for FormatCount (enum)
macro_rules! Depcrate_formatFormatCount {
() => {
// Module: crate::format
// Provides: {"FormatCount"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , PartialEq , Eq)] pub enum FormatCount { # [doc = " `{:5}` or `{:.5}`"] Literal (u16) , # [doc = " `{:.*}`, `{:.5$}`, or `{:a$}`, etc."] Argument (FormatArgPosition) , }
};
}
