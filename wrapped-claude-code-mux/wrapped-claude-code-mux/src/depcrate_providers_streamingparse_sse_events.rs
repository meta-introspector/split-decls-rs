// Generated macro for parse_sse_events (function)
macro_rules! Depcrate_providers_streamingparse_sse_events {
() => {
// Module: crate::providers::streaming
// Provides: {"parse_sse_events"}
// Dependencies: {}
# [doc = " Parse SSE events from a byte stream"] pub fn parse_sse_events (input : & str) -> Vec < SseEvent > { let mut events = Vec :: new () ; let mut current_event : Option < String > = None ; let mut current_data = String :: new () ; for line in input . lines () { if line . is_empty () { if ! current_data . is_empty () { events . push (SseEvent { event : current_event . take () , data : current_data . clone () , }) ; current_data . clear () ; } } else if let Some (data) = line . strip_prefix ("data: ") { if ! current_data . is_empty () { current_data . push ('\n') ; } current_data . push_str (data) ; } else if let Some (event) = line . strip_prefix ("event: ") { current_event = Some (event . to_string ()) ; } } if ! current_data . is_empty () { events . push (SseEvent { event : current_event , data : current_data , }) ; } events }
};
}
