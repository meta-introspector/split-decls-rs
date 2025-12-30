// Generated macro for Dep (struct)
macro_rules! Depcrate_project_jsonDep {
() => {
// Module: crate::project_json
// Provides: {"Dep"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone , Eq , PartialEq)] pub (crate) struct Dep { # [doc = " Identifies a crate by position in the crates array."] # [serde (rename = "crate")] pub (crate) krate : CrateArrayIdx , # [serde (serialize_with = "serialize_crate_name")] # [serde (deserialize_with = "deserialize_crate_name")] pub (crate) name : CrateName , }
};
}
