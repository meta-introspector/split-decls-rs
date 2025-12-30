// Generated macro for StyleOption (enum)
macro_rules! Depcrate_export_serializers_jsonStyleOption {
() => {
// Module: crate::export::serializers::json
// Provides: {"StyleOption"}
// Dependencies: {}
# [doc = " Choices for how to render the JSON files."] # [non_exhaustive] # [derive (Copy , Clone , Debug , PartialEq , Default)] pub enum StyleOption { # [doc = " Print the smallest possible JSON, to reduce file size."] # [default] Compact , # [doc = " Pretty-print JSON, to make it more readable."] Pretty , }
};
}
