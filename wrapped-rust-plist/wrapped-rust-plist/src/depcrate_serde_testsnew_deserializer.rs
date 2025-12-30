// Generated macro for new_deserializer (function)
macro_rules! Depcrate_serde_testsnew_deserializer {
() => {
// Module: crate::serde_tests
// Provides: {"new_deserializer"}
// Dependencies: {}
fn new_deserializer (events : Vec < Event >) -> Deserializer < Vec < Result < Event , Error > > > { let result_events = events . into_iter () . map (Ok) . collect () ; Deserializer :: new (result_events) }
};
}
