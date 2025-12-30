// Generated macro for stream_cargo (function)
macro_rules! Depcrate_core_build_steps_compilestream_cargo {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"stream_cargo"}
// Dependencies: {}
pub fn stream_cargo (builder : & Builder < '_ > , cargo : Cargo , tail_args : Vec < String > , cb : & mut dyn FnMut (CargoMessage < '_ >) ,) -> bool { let mut cmd = cargo . into_cmd () ; let mut message_format = if builder . config . json_output { String :: from ("json") } else { String :: from ("json-render-diagnostics") } ; if let Some (s) = & builder . config . rustc_error_format { message_format . push_str (",json-diagnostic-") ; message_format . push_str (s) ; } cmd . arg ("--message-format") . arg (message_format) ; for arg in tail_args { cmd . arg (arg) ; } builder . verbose (| | println ! ("running: {cmd:?}")) ; let streaming_command = cmd . stream_capture_stdout (& builder . config . exec_ctx) ; let Some (mut streaming_command) = streaming_command else { return true ; } ; let stdout = BufReader :: new (streaming_command . stdout . take () . unwrap ()) ; for line in stdout . lines () { let line = t ! (line) ; match serde_json :: from_str :: < CargoMessage < '_ > > (& line) { Ok (msg) => { if builder . config . json_output { println ! ("{line}") ; } cb (msg) } Err (_) => println ! ("{line}") , } } let status = t ! (streaming_command . wait (& builder . config . exec_ctx)) ; if builder . is_verbose () && ! status . success () { eprintln ! ("command did not execute successfully: {cmd:?}\n\
                  expected success, got: {status}") ; } status . success () }
};
}
