// Generated macro for load_env_var (function)
macro_rules! Depcrate_utilsload_env_var {
() => {
// Module: crate::utils
// Provides: {"load_env_var"}
// Dependencies: {}
pub fn load_env_var (name : & str) -> anyhow :: Result < String > { std :: env :: var (name) . with_context (| | format ! ("Cannot find environment variable `{name}`")) }
};
}
