// Generated macro for FormatArgPositionKind (enum)
macro_rules! Depcrate_formatFormatArgPositionKind {
() => {
// Module: crate::format
// Provides: {"FormatArgPositionKind"}
// Dependencies: {}
# [derive (Copy , Clone , Encodable , Decodable , Debug , PartialEq , Eq)] pub enum FormatArgPositionKind { # [doc = " `{}` or `{:.*}`"] Implicit , # [doc = " `{1}` or `{:1$}` or `{:.1$}`"] Number , # [doc = " `{a}` or `{:a$}` or `{:.a$}`"] Named , }
};
}
