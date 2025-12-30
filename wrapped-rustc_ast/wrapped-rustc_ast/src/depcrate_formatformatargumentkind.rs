// Generated macro for FormatArgumentKind (enum)
macro_rules! Depcrate_formatFormatArgumentKind {
() => {
// Module: crate::format
// Provides: {"FormatArgumentKind"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum FormatArgumentKind { # [doc = " `format_args(…, arg)`"] Normal , # [doc = " `format_args(…, arg = 1)`"] Named (Ident) , # [doc = " `format_args(\"… {arg} …\")`"] Captured (Ident) , }
};
}
