// Generated macro for pseudo_instant_event (function)
macro_rules! Depcrate_testing_commonpseudo_instant_event {
() => {
// Module: crate::testing_common
// Provides: {"pseudo_instant_event"}
// Dependencies: {}
fn pseudo_instant_event (profiler : & Profiler , random : usize , thread_id : u32 , event_ids : & [(StringId , EventId)] , expected_events_templates : & [ExpectedEvent] , expected_events : & mut Vec < Event < 'static > > ,) { let random_event_index = random % event_ids . len () ; let (event_kind , event_id) = event_ids [random_event_index] ; profiler . record_instant_event (event_kind , event_id , thread_id) ; expected_events . push (Event { event_kind : expected_events_templates [random_event_index] . kind . clone () , label : expected_events_templates [random_event_index] . label . clone () , additional_data : expected_events_templates [random_event_index] . args . clone () , thread_id , payload : EventPayload :: Timestamp (Timestamp :: Instant (SystemTime :: UNIX_EPOCH)) , }) ; }
};
}
