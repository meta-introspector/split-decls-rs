// Generated macro for env_path (function)
macro_rules! Depcrateenv_path {
() => {
// Module: crate
// Provides: {"env_path"}
// Dependencies: {}
# [doc = " Grab an environment variable as a PathBuf, or fail nicely."] fn env_path (var : & str) -> Result < PathBuf , Error > { if let Some (var) = std :: env :: var_os (var) { Ok (var . into ()) } else { anyhow :: bail ! ("missing environment variable {var}") } }
};
}
