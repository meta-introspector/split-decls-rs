// Generated macro for is_docker (function)
macro_rules! Depcrateis_docker {
() => {
// Module: crate
// Provides: {"is_docker"}
// Dependencies: {}
pub fn is_docker () -> bool { static CACHED_RESULT : OnceCell < bool > = OnceCell :: new () ; * CACHED_RESULT . get_or_init (| | { has_docker_env_file () || has_docker_in_cgroup () }) }
};
}
