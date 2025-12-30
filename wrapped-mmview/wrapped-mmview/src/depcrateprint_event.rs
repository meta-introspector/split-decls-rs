// Generated macro for print_event (function)
macro_rules! Depcrateprint_event {
() => {
// Module: crate
// Provides: {"print_event"}
// Dependencies: {}
fn print_event (event : & Event < '_ > , global_start_time : SystemTime) { let additional_data = event . additional_data . join (",") ; let payload = match event . payload { EventPayload :: Timestamp (Timestamp :: Instant (t)) => { format ! ("{} μs" , system_time_to_micros_since (t , global_start_time)) } EventPayload :: Timestamp (Timestamp :: Interval { start , end }) => format ! ("{} μs - {} μs" , system_time_to_micros_since (start , global_start_time) , system_time_to_micros_since (end , global_start_time)) , EventPayload :: Integer (i) => format ! ("{}" , i) , } ; println ! (r#"{{
    kind: {},
    label: {},
    additional_data: [{}],
    payload: {},
    thread_id: {},
}}"# , event . event_kind , event . label , additional_data , payload , event . thread_id) ; }
};
}
