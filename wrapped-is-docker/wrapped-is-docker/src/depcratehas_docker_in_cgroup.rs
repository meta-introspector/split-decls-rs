// Generated macro for has_docker_in_cgroup (function)
macro_rules! Depcratehas_docker_in_cgroup {
() => {
// Module: crate
// Provides: {"has_docker_in_cgroup"}
// Dependencies: {}
fn has_docker_in_cgroup () -> bool { match fs :: read_to_string ("/proc/self/cgroup") { Ok (file_contents) => file_contents . contains ("docker") , Err (_error) => false , } }
};
}
