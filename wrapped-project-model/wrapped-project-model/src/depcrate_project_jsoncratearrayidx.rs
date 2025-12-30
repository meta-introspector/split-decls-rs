// Generated macro for CrateArrayIdx (struct)
macro_rules! Depcrate_project_jsonCrateArrayIdx {
() => {
// Module: crate::project_json
// Provides: {"CrateArrayIdx"}
// Dependencies: {}
# [doc = " Identifies a crate by position in the crates array."] # [doc = ""] # [doc = " This will differ from `Crate` when multiple `ProjectJson`"] # [doc = " workspaces are loaded."] # [derive (Serialize , Deserialize , Debug , Clone , Copy , Eq , PartialEq , Hash)] # [serde (transparent)] pub struct CrateArrayIdx (pub usize) ;
};
}
