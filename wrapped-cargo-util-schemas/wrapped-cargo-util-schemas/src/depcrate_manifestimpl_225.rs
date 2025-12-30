// Generated macro for impl_225 (impl)
macro_rules! Depcrate_manifestimpl_225 {
() => {
// Module: crate::manifest
// Provides: {"impl_225"}
// Dependencies: {}
impl TomlLint { pub fn level (& self) -> TomlLintLevel { match self { Self :: Level (level) => * level , Self :: Config (config) => config . level , } } pub fn priority (& self) -> i8 { match self { Self :: Level (_) => 0 , Self :: Config (config) => config . priority , } } pub fn config (& self) -> Option < & toml :: Table > { match self { Self :: Level (_) => None , Self :: Config (config) => Some (& config . config) , } } }
};
}
