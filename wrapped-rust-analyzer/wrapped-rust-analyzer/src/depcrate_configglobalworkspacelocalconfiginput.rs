// Generated macro for GlobalWorkspaceLocalConfigInput (struct)
macro_rules! Depcrate_configGlobalWorkspaceLocalConfigInput {
() => {
// Module: crate::config
// Provides: {"GlobalWorkspaceLocalConfigInput"}
// Dependencies: {}
# [doc = " All of the config levels, all fields `Option<T>`, to describe fields that are actually set by"] # [doc = " some rust-analyzer.toml file or JSON blob. An empty rust-analyzer.toml corresponds to"] # [doc = " all fields being None."] # [derive (Debug , Clone , Default)] struct GlobalWorkspaceLocalConfigInput { global : GlobalConfigInput , local : LocalConfigInput , workspace : WorkspaceConfigInput , }
};
}
