// Generated macro for FormatPlaceholder (struct)
macro_rules! Depcrate_formatFormatPlaceholder {
() => {
// Module: crate::format
// Provides: {"FormatPlaceholder"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , PartialEq , Eq , Walkable)] pub struct FormatPlaceholder { # [doc = " Index into [`FormatArgs::arguments`]."] pub argument : FormatArgPosition , # [doc = " The span inside the format string for the full `{…}` placeholder."] pub span : Option < Span > , # [doc = " `{}`, `{:?}`, or `{:x}`, etc."] # [visitable (ignore)] pub format_trait : FormatTrait , # [doc = " `{}` or `{:.5}` or `{:-^20}`, etc."] # [visitable (ignore)] pub format_options : FormatOptions , }
};
}
