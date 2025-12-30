// Generated macro for has_docker_env_file (function)
macro_rules! Depcratehas_docker_env_file {
() => {
// Module: crate
// Provides: {"has_docker_env_file"}
// Dependencies: {}
fn has_docker_env_file () -> bool { fs :: metadata ("/.dockerenv") . is_ok () }
};
}
