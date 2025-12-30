// Generated macro for trace_cmd (function)
macro_rules! Depcrate_utils_tracingtrace_cmd {
() => {
// Module: crate::utils::tracing
// Provides: {"trace_cmd"}
// Dependencies: {}
# [cfg (feature = "tracing")] pub fn trace_cmd (command : & crate :: BootstrapCommand) -> tracing :: span :: EnteredSpan { let fingerprint = command . fingerprint () ; let location = command . get_created_location () ; let location = format_location (location) ; tracing :: span ! (target : COMMAND_SPAN_TARGET , tracing :: Level :: TRACE , "cmd" , cmd_name = fingerprint . program_name () . to_string () , cmd = fingerprint . format_short_cmd () , full_cmd = ? command , location) . entered () }
};
}
