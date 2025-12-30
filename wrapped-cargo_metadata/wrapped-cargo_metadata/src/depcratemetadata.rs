// Generated macro for Metadata (struct)
macro_rules! DepcrateMetadata {
() => {
// Module: crate
// Provides: {"Metadata"}
// Dependencies: {}
# [derive (Clone , Serialize , Deserialize , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "builder" , derive (Builder))] # [non_exhaustive] # [cfg_attr (feature = "builder" , builder (pattern = "owned" , setter (into)))] # [doc = " Starting point for metadata returned by `cargo metadata`"] pub struct Metadata { # [doc = " A list of all crates referenced by this crate (and the crate itself)"] pub packages : Vec < Package > , # [doc = " A list of all workspace members"] pub workspace_members : Vec < PackageId > , # [doc = " The list of default workspace members"] # [doc = ""] # [doc = " This is not available if running with a version of Cargo older than 1.71."] # [doc = ""] # [doc = " You can check whether it is available or missing using respectively"] # [doc = " [`WorkspaceDefaultMembers::is_available`] and [`WorkspaceDefaultMembers::is_missing`]."] # [serde (default , skip_serializing_if = "WorkspaceDefaultMembers::is_missing")] pub workspace_default_members : WorkspaceDefaultMembers , # [doc = " Dependencies graph"] pub resolve : Option < Resolve > , # [doc = " Workspace root"] pub workspace_root : Utf8PathBuf , # [doc = " Target directory"] pub target_directory : Utf8PathBuf , # [doc = " Build directory"] pub build_directory : Option < Utf8PathBuf > , # [doc = " The workspace-level metadata object. Null if non-existent."] # [serde (rename = "metadata" , default , skip_serializing_if = "is_null")] pub workspace_metadata : serde_json :: Value , # [doc = " The metadata format version"] version : usize , }
};
}
