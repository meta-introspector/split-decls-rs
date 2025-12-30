// Generated macro for FormatArgsPiece (enum)
macro_rules! Depcrate_formatFormatArgsPiece {
() => {
// Module: crate::format
// Provides: {"FormatArgsPiece"}
// Dependencies: {}
# [doc = " A piece of a format template string."] # [doc = ""] # [doc = " E.g. \"hello\" or \"{name}\"."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum FormatArgsPiece { Literal (Symbol) , Placeholder (FormatPlaceholder) , }
};
}
