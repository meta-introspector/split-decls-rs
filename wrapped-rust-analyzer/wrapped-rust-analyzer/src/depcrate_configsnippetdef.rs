// Generated macro for SnippetDef (struct)
macro_rules! Depcrate_configSnippetDef {
() => {
// Module: crate::config
// Provides: {"SnippetDef"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone , Default)] # [serde (default)] pub (crate) struct SnippetDef { # [serde (with = "single_or_array")] # [serde (skip_serializing_if = "Vec::is_empty")] prefix : Vec < String > , # [serde (with = "single_or_array")] # [serde (skip_serializing_if = "Vec::is_empty")] postfix : Vec < String > , # [serde (with = "single_or_array")] # [serde (skip_serializing_if = "Vec::is_empty")] body : Vec < String > , # [serde (with = "single_or_array")] # [serde (skip_serializing_if = "Vec::is_empty")] requires : Vec < String > , # [serde (skip_serializing_if = "Option::is_none")] description : Option < String > , scope : SnippetScopeDef , }
};
}
