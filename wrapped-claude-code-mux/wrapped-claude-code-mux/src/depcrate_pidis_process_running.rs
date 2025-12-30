// Generated macro for is_process_running (function)
macro_rules! Depcrate_pidis_process_running {
() => {
// Module: crate::pid
// Provides: {"is_process_running"}
// Dependencies: {}
# [cfg (windows)] pub fn is_process_running (pid : u32) -> bool { use std :: process :: Command ; Command :: new ("tasklist") . args (& ["/FI" , & format ! ("PID eq {}" , pid)]) . output () . ok () . and_then (| output | { String :: from_utf8 (output . stdout) . ok () . map (| s | s . contains (& pid . to_string ())) }) . unwrap_or (false) }
};
}
