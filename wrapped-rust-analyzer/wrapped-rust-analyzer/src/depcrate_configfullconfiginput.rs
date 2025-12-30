// Generated macro for FullConfigInput (struct)
macro_rules! Depcrate_configFullConfigInput {
() => {
// Module: crate::config
// Provides: {"FullConfigInput"}
// Dependencies: {}
# [doc = " All of the config levels, all fields `Option<T>`, to describe fields that are actually set by"] # [doc = " some rust-analyzer.toml file or JSON blob. An empty rust-analyzer.toml corresponds to"] # [doc = " all fields being None."] # [derive (Debug , Clone , Default)] struct FullConfigInput { global : GlobalConfigInput , workspace : WorkspaceConfigInput , local : LocalConfigInput , client : ClientConfigInput , }
};
}
