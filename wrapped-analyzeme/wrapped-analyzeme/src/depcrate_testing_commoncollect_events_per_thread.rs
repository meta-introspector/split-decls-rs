// Generated macro for collect_events_per_thread (function)
macro_rules! Depcrate_testing_commoncollect_events_per_thread {
() => {
// Module: crate::testing_common
// Provides: {"collect_events_per_thread"}
// Dependencies: {}
fn collect_events_per_thread < 'a > (events : & mut dyn Iterator < Item = Event < 'a > > ,) -> FxHashMap < u32 , Vec < Event < 'a > > > { let mut per_thread : FxHashMap < _ , _ > = Default :: default () ; for event in events { per_thread . entry (event . thread_id) . or_insert (Vec :: new ()) . push (event) ; } per_thread }
};
}
