// Generated macro for impl_351 (impl)
macro_rules! Depcrate_configimpl_351 {
() => {
// Module: crate::config
// Provides: {"impl_351"}
// Dependencies: {}
impl GlobalWorkspaceLocalConfigInput { const FIELDS : & 'static [& 'static [& 'static str]] = & [GlobalConfigInput :: FIELDS , LocalConfigInput :: FIELDS] ; fn from_toml (toml : toml :: Table , error_sink : & mut Vec < (String , toml :: de :: Error) > ,) -> GlobalWorkspaceLocalConfigInput { GlobalWorkspaceLocalConfigInput { global : GlobalConfigInput :: from_toml (& toml , error_sink) , local : LocalConfigInput :: from_toml (& toml , error_sink) , workspace : WorkspaceConfigInput :: from_toml (& toml , error_sink) , } } }
};
}
