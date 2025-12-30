// Generated macro for ProjectJsonData (struct)
macro_rules! Depcrate_project_jsonProjectJsonData {
() => {
// Module: crate::project_json
// Provides: {"ProjectJsonData"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone , Eq , PartialEq)] pub struct ProjectJsonData { sysroot : Option < Utf8PathBuf > , sysroot_src : Option < Utf8PathBuf > , sysroot_project : Option < Box < ProjectJsonData > > , # [serde (default)] cfg_groups : FxHashMap < String , CfgList > , crates : Vec < CrateData > , # [serde (default)] runnables : Vec < RunnableData > , }
};
}
