// Generated macro for default_path (function)
macro_rules! Depcrate_core_build_steps_installdefault_path {
() => {
// Module: crate::core::build_steps::install
// Provides: {"default_path"}
// Dependencies: {}
fn default_path (config : & Option < PathBuf > , default : & str) -> PathBuf { config . as_ref () . cloned () . unwrap_or_else (| | PathBuf :: from (default)) }
};
}
