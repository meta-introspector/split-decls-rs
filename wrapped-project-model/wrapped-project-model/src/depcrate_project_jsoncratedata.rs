// Generated macro for CrateData (struct)
macro_rules! Depcrate_project_jsonCrateData {
() => {
// Module: crate::project_json
// Provides: {"CrateData"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone , Eq , PartialEq)] struct CrateData { display_name : Option < String > , root_module : Utf8PathBuf , edition : EditionData , # [serde (default)] version : Option < semver :: Version > , deps : Vec < Dep > , # [serde (default)] cfg_groups : FxHashSet < String > , # [serde (default)] cfg : CfgList , target : Option < String > , # [serde (default)] env : FxHashMap < String , String > , proc_macro_dylib_path : Option < Utf8PathBuf > , is_workspace_member : Option < bool > , source : Option < CrateSource > , # [serde (default)] is_proc_macro : bool , # [serde (default)] repository : Option < String > , # [serde (default)] build : Option < BuildData > , # [serde (default)] proc_macro_cwd : Option < Utf8PathBuf > , }
};
}
