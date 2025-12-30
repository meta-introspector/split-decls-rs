// Generated macro for impl_340 (impl)
macro_rules! Depcrate_configimpl_340 {
() => {
// Module: crate::config
// Provides: {"impl_340"}
// Dependencies: {}
impl WorkspaceLocalConfigInput { # [allow (dead_code)] const FIELDS : & 'static [& 'static [& 'static str]] = & [WorkspaceConfigInput :: FIELDS , LocalConfigInput :: FIELDS] ; fn from_toml (toml : toml :: Table , error_sink : & mut Vec < (String , toml :: de :: Error) >) -> Self { Self { workspace : WorkspaceConfigInput :: from_toml (& toml , error_sink) , local : LocalConfigInput :: from_toml (& toml , error_sink) , } } }
};
}
