// Generated macro for is_wsl (function)
macro_rules! Depcrateis_wsl {
() => {
// Module: crate
// Provides: {"is_wsl"}
// Dependencies: {}
pub fn is_wsl () -> bool { static CACHED_RESULT : OnceCell < bool > = OnceCell :: new () ; * CACHED_RESULT . get_or_init (| | { if std :: env :: consts :: OS != "linux" { return false ; } if let Ok (os_release) = get_os_release () { if os_release . to_lowercase () . contains ("microsoft") { return ! is_docker :: is_docker () ; } } if proc_version_includes_microsoft () { ! is_docker :: is_docker () } else { false } }) }
};
}
