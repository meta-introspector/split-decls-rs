// Generated macro for WorkspaceDefaultMembers (struct)
macro_rules! DepcrateWorkspaceDefaultMembers {
() => {
// Module: crate
// Provides: {"WorkspaceDefaultMembers"}
// Dependencies: {}
# [derive (Clone , Debug , Deserialize , Serialize , PartialEq , Eq , Hash , Default)] # [serde (transparent)] # [doc = " A list of default workspace members."] # [doc = ""] # [doc = " See [`Metadata::workspace_default_members`]."] # [doc = ""] # [doc = " It is only available if running a version of Cargo of 1.71 or newer."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Dereferencing when running an older version of Cargo will panic."] pub struct WorkspaceDefaultMembers (Option < Vec < PackageId > >) ;
};
}
