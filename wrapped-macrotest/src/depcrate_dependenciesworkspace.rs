// Generated macro for Workspace (struct)
macro_rules! Depcrate_dependenciesWorkspace {
() => {
// Module: crate::dependencies
// Provides: {"Workspace"}
// Dependencies: {}
# [derive (Deserialize , Default , Debug)] pub struct Workspace { # [serde (default)] pub package : WorkspacePackage , # [serde (default)] pub dependencies : Map < String , Dependency > , }
};
}
