// Generated macro for check_profiling_data (function)
macro_rules! Depcrate_testing_commoncheck_profiling_data {
() => {
// Module: crate::testing_common
// Provides: {"check_profiling_data"}
// Dependencies: {}
fn check_profiling_data (actual_events : & mut dyn Iterator < Item = Event < '_ > > , expected_events : & mut dyn Iterator < Item = Event < '_ > > , num_expected_events : usize ,) { let mut count = 0 ; assert_eq ! ((num_expected_events , Some (num_expected_events)) , actual_events . size_hint ()) ; let actual_events_per_thread = collect_events_per_thread (actual_events) ; let expected_events_per_thread = collect_events_per_thread (expected_events) ; let thread_ids : Vec < _ > = actual_events_per_thread . keys () . collect () ; assert_eq ! (thread_ids , expected_events_per_thread . keys () . collect ::< Vec < _ >> ()) ; for thread_id in thread_ids { let actual_events = & actual_events_per_thread [thread_id] ; let expected_events = & expected_events_per_thread [thread_id] ; assert_eq ! (actual_events . len () , expected_events . len ()) ; for (actual_event , expected_event) in actual_events . iter () . zip (expected_events . iter ()) { assert_eq ! (actual_event . event_kind , expected_event . event_kind) ; assert_eq ! (actual_event . label , expected_event . label) ; assert_eq ! (actual_event . additional_data , expected_event . additional_data) ; assert_eq ! (actual_event . payload . is_interval () , expected_event . payload . is_interval ()) ; assert_eq ! (actual_event . payload . is_instant () , expected_event . payload . is_instant ()) ; if expected_event . payload . is_integer () { assert_eq ! (actual_event . payload , expected_event . payload) ; } count += 1 ; } } assert_eq ! (count , num_expected_events) ; }
};
}
