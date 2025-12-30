// Generated macro for clean_json_schema (function)
macro_rules! Depcrate_providers_geminiclean_json_schema {
() => {
// Module: crate::providers::gemini
// Provides: {"clean_json_schema"}
// Dependencies: {}
# [doc = " Remove JSON Schema metadata fields that Gemini API doesn't support"] fn clean_json_schema (value : & mut serde_json :: Value) { match value { serde_json :: Value :: Object (map) => { map . remove ("$schema") ; map . remove ("$id") ; map . remove ("$ref") ; map . remove ("$comment") ; map . remove ("exclusiveMinimum") ; map . remove ("exclusiveMaximum") ; map . remove ("definitions") ; map . remove ("$defs") ; for (_ , v) in map . iter_mut () { clean_json_schema (v) ; } } serde_json :: Value :: Array (arr) => { for item in arr . iter_mut () { clean_json_schema (item) ; } } _ => { } } }
};
}
