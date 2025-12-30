// Generated macro for pseudo_integer_event (function)
macro_rules! Depcrate_testing_commonpseudo_integer_event {
() => {
// Module: crate::testing_common
// Provides: {"pseudo_integer_event"}
// Dependencies: {}
fn pseudo_integer_event (profiler : & Profiler , random : usize , thread_id : u32 , event_ids : & [(StringId , EventId)] , expected_events_templates : & [ExpectedEvent] , expected_events : & mut Vec < Event < 'static > > ,) { let random_event_index = random % event_ids . len () ; let payload_value = random as u64 * 33 ; let (event_kind , event_id) = event_ids [random_event_index] ; profiler . record_integer_event (event_kind , event_id , thread_id , payload_value) ; expected_events . push (Event { event_kind : expected_events_templates [random_event_index] . kind . clone () , label : expected_events_templates [random_event_index] . label . clone () , additional_data : expected_events_templates [random_event_index] . args . clone () , thread_id , payload : EventPayload :: Integer (payload_value) , }) ; }
};
}
