// Generated macro for WorkspaceLocalConfigInput (struct)
macro_rules! Depcrate_configWorkspaceLocalConfigInput {
() => {
// Module: crate::config
// Provides: {"WorkspaceLocalConfigInput"}
// Dependencies: {}
# [doc = " Workspace and local config levels, all fields `Option<T>`, to describe fields that are actually set by"] # [doc = " some rust-analyzer.toml file or JSON blob. An empty rust-analyzer.toml corresponds to"] # [doc = " all fields being None."] # [derive (Debug , Clone , Default)] # [allow (dead_code)] struct WorkspaceLocalConfigInput { workspace : WorkspaceConfigInput , local : LocalConfigInput , }
};
}
