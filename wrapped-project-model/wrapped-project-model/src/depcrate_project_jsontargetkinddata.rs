// Generated macro for TargetKindData (enum)
macro_rules! Depcrate_project_jsonTargetKindData {
() => {
// Module: crate::project_json
// Provides: {"TargetKindData"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , PartialEq , Eq , Deserialize , Serialize)] # [serde (rename_all = "camelCase")] pub enum TargetKindData { Bin , # [doc = " Any kind of Cargo lib crate-type (dylib, rlib, proc-macro, ...)."] Lib , Test , }
};
}
