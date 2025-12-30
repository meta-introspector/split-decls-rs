// Generated macro for run_tests (function)
macro_rules! Depcrate_utils_render_testsrun_tests {
() => {
// Module: crate::utils::render_tests
// Provides: {"run_tests"}
// Dependencies: {}
fn run_tests (builder : & Builder < '_ > , cmd : & mut BootstrapCommand , stream : bool) -> bool { builder . verbose (| | println ! ("running: {cmd:?}")) ; let Some (mut streaming_command) = cmd . stream_capture_stdout (& builder . config . exec_ctx) else { return true ; } ; let renderer = Renderer :: new (streaming_command . stdout . take () . unwrap () , builder) ; if stream { renderer . stream_all () ; } else { renderer . render_all () ; } let status = streaming_command . wait (& builder . config . exec_ctx) . unwrap () ; if ! status . success () && builder . is_verbose () { println ! ("\n\ncommand did not execute successfully: {cmd:?}\n\
             expected success, got: {status}" ,) ; } status . success () }
};
}
