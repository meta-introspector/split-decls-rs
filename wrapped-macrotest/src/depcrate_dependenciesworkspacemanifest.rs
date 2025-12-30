// Generated macro for WorkspaceManifest (struct)
macro_rules! Depcrate_dependenciesWorkspaceManifest {
() => {
// Module: crate::dependencies
// Provides: {"WorkspaceManifest"}
// Dependencies: {}
# [derive (Deserialize , Default , Debug)] pub struct WorkspaceManifest { # [serde (default)] pub workspace : Workspace , # [serde (default)] pub patch : Map < String , RegistryPatch > , # [serde (default)] pub replace : Map < String , Patch > , }
};
}
