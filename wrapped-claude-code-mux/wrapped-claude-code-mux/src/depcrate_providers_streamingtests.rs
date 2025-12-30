// Generated macro for tests (module)
macro_rules! Depcrate_providers_streamingtests {
() => {
// Module: crate::providers::streaming
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_parse_sse_single_event () { let input = "event: message\ndata: {\"test\":\"value\"}\n\n" ; let events = parse_sse_events (input) ; assert_eq ! (events . len () , 1) ; assert_eq ! (events [0] . event . as_deref () , Some ("message")) ; assert_eq ! (events [0] . data , "{\"test\":\"value\"}") ; } # [test] fn test_parse_sse_multiple_events () { let input = "event: start\ndata: {\"a\":1}\n\nevent: delta\ndata: {\"b\":2}\n\n" ; let events = parse_sse_events (input) ; assert_eq ! (events . len () , 2) ; assert_eq ! (events [0] . event . as_deref () , Some ("start")) ; assert_eq ! (events [1] . event . as_deref () , Some ("delta")) ; } # [test] fn test_parse_sse_no_event_type () { let input = "data: plain data\n\n" ; let events = parse_sse_events (input) ; assert_eq ! (events . len () , 1) ; assert ! (events [0] . event . is_none ()) ; assert_eq ! (events [0] . data , "plain data") ; } }
};
}
