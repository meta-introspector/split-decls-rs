// Generated macro for proc_version_includes_microsoft (function)
macro_rules! Depcrateproc_version_includes_microsoft {
() => {
// Module: crate
// Provides: {"proc_version_includes_microsoft"}
// Dependencies: {}
fn proc_version_includes_microsoft () -> bool { match std :: fs :: read_to_string ("/proc/version") { Ok (file_contents) => file_contents . to_lowercase () . contains ("microsoft") , Err (_) => false , } }
};
}
