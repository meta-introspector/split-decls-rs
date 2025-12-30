// Generated macro for parse_cli (function)
macro_rules! Depcrate_cliparse_cli {
() => {
// Module: crate::cli
// Provides: {"parse_cli"}
// Dependencies: {}
pub fn parse_cli () -> anyhow :: Result < Args > { let app = Args :: command () ; let app = app . name (std :: env :: current_exe () ? . file_name () . and_then (| s | s . to_str ()) . map (| s | s . to_owned ()) . expect ("Binary name not found") ,) ; Ok (Args :: from_arg_matches (& app . get_matches ()) ?) }
};
}
