// Generated macro for AutoImportExclusion (enum)
macro_rules! Depcrate_configAutoImportExclusion {
() => {
// Module: crate::config
// Provides: {"AutoImportExclusion"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone)] # [serde (untagged)] # [serde (rename_all = "snake_case")] pub enum AutoImportExclusion { Path (String) , Verbose { path : String , r#type : AutoImportExclusionType } , }
};
}
