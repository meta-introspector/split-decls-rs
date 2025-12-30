// Generated macro for MethodData (struct)
macro_rules! Depcrate_configMethodData {
() => {
// Module: crate::config
// Provides: {"MethodData"}
// Dependencies: {}
# [derive (Deserialize , Debug , Default , Clone , PartialEq , Eq)] # [serde (deny_unknown_fields)] pub struct MethodData { # [serde (default)] pub skipped : bool , # [serde (default)] pub renamed : Option < String > , # [serde (rename = "unsafe")] # [serde (default)] pub unsafe_ : Option < bool > , # [serde (default)] # [serde (deserialize_with = "deserialize_argument_overrides")] pub arguments : HashMap < usize , TypeOverride > , # [serde (rename = "return")] # [serde (default)] pub return_ : TypeOverride , }
};
}
