// Generated macro for TargetDirectory (enum)
macro_rules! Depcrate_configTargetDirectory {
() => {
// Module: crate::config
// Provides: {"TargetDirectory"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone , PartialEq)] # [serde (rename_all = "snake_case")] # [serde (untagged)] pub enum TargetDirectory { UseSubdirectory (bool) , Directory (Utf8PathBuf) , }
};
}
