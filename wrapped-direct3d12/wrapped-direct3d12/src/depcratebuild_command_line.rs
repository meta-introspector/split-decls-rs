// Generated macro for build_command_line (function)
macro_rules! Depcratebuild_command_line {
() => {
// Module: crate
// Provides: {"build_command_line"}
// Dependencies: {}
fn build_command_line () -> SampleCommandLine { let mut use_warp_device = false ; for arg in std :: env :: args () { if arg . eq_ignore_ascii_case ("-warp") || arg . eq_ignore_ascii_case ("/warp") { use_warp_device = true ; } } SampleCommandLine { use_warp_device } }
};
}
