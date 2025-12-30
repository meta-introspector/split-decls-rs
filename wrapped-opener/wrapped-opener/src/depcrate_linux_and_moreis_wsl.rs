// Generated macro for is_wsl (function)
macro_rules! Depcrate_linux_and_moreis_wsl {
() => {
// Module: crate::linux_and_more
// Provides: {"is_wsl"}
// Dependencies: {}
# [cfg (target_os = "linux")] pub (crate) fn is_wsl () -> bool { if is_docker () { return false ; } if let Ok (true) = std :: fs :: read_to_string ("/proc/sys/kernel/osrelease") . map (| osrelease | osrelease . to_ascii_lowercase () . contains ("microsoft")) { return true ; } if let Ok (true) = std :: fs :: read_to_string ("/proc/version") . map (| version | version . to_ascii_lowercase () . contains ("microsoft")) { return true ; } false }
};
}
