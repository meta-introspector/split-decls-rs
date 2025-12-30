// Generated macro for pseudo_invocation (function)
macro_rules! Depcrate_testing_commonpseudo_invocation {
() => {
// Module: crate::testing_common
// Provides: {"pseudo_invocation"}
// Dependencies: {}
fn pseudo_invocation (profiler : & Profiler , random : usize , thread_id : u32 , recursions_left : usize , event_ids : & [(StringId , EventId)] , expected_events_templates : & [ExpectedEvent] , expected_events : & mut Vec < Event < 'static > > ,) { if recursions_left == 0 { return ; } let random_event_index = random % event_ids . len () ; let (event_kind , event_id) = event_ids [random_event_index] ; let _prof_guard = profiler . start_recording_interval_event (event_kind , event_id , thread_id) ; pseudo_integer_event (profiler , random * 7 , thread_id , event_ids , expected_events_templates , expected_events ,) ; pseudo_invocation (profiler , random * 17 , thread_id , recursions_left - 1 , event_ids , expected_events_templates , expected_events ,) ; pseudo_instant_event (profiler , random * 23 , thread_id , event_ids , expected_events_templates , expected_events ,) ; expected_events . push (Event { event_kind : expected_events_templates [random_event_index] . kind . clone () , label : expected_events_templates [random_event_index] . label . clone () , additional_data : expected_events_templates [random_event_index] . args . clone () , thread_id , payload : EventPayload :: Timestamp (Timestamp :: Interval { start : SystemTime :: UNIX_EPOCH , end : SystemTime :: UNIX_EPOCH , }) , }) ; }
};
}
