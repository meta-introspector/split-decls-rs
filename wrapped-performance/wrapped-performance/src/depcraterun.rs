// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
# [wasm_bindgen (start)] fn run () { let window = web_sys :: window () . expect ("should have a window in this context") ; let performance = window . performance () . expect ("performance should be available") ; console_log ! ("the current time (in ms) is {}" , performance . now ()) ; let start = perf_to_system (performance . timing () . request_start ()) ; let end = perf_to_system (performance . timing () . response_end ()) ; console_log ! ("request started at {}" , humantime :: format_rfc3339 (start)) ; console_log ! ("request ended at {}" , humantime :: format_rfc3339 (end)) ; }
};
}
