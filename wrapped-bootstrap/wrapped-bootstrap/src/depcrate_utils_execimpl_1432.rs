// Generated macro for impl_1432 (impl)
macro_rules! Depcrate_utils_execimpl_1432 {
() => {
// Module: crate::utils::exec
// Provides: {"impl_1432"}
// Dependencies: {}
impl CommandFingerprint { # [cfg (feature = "tracing")] pub (crate) fn program_name (& self) -> String { Path :: new (& self . program) . file_name () . map (| p | p . to_string_lossy () . to_string ()) . unwrap_or_else (| | "<unknown command>" . to_string ()) } # [doc = " Helper method to format both Command and BootstrapCommand as a short execution line,"] # [doc = " without all the other details (e.g. environment variables)."] pub (crate) fn format_short_cmd (& self) -> String { use std :: fmt :: Write ; let mut cmd = self . program . to_string_lossy () . to_string () ; for arg in & self . args { let arg = arg . to_string_lossy () ; if arg . contains (' ') { write ! (cmd , " '{arg}'") . unwrap () ; } else { write ! (cmd , " {arg}") . unwrap () ; } } if let Some (cwd) = & self . cwd { write ! (cmd , " [workdir={}]" , cwd . to_string_lossy ()) . unwrap () ; } cmd } }
};
}
