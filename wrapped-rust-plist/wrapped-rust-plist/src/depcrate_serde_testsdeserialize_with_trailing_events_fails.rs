// Generated macro for deserialize_with_trailing_events_fails (function)
macro_rules! Depcrate_serde_testsdeserialize_with_trailing_events_fails {
() => {
// Module: crate::serde_tests
// Provides: {"deserialize_with_trailing_events_fails"}
// Dependencies: {}
# [test] fn deserialize_with_trailing_events_fails () { let events = [Event :: String ("Foo" . into ()) , Event :: String ("Bar" . into ())] ; let value = crate :: de :: from_stream :: < Value > (events . map (Ok)) ; assert ! (value . is_err ()) ; }
};
}
