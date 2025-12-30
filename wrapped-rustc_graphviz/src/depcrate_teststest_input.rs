// Generated macro for test_input (function)
macro_rules! Depcrate_teststest_input {
() => {
// Module: crate::tests
// Provides: {"test_input"}
// Dependencies: {}
fn test_input (g : LabelledGraph) -> io :: Result < String > { let mut writer = Vec :: new () ; render (& g , & mut writer) . unwrap () ; let mut s = String :: new () ; Read :: read_to_string (& mut & * writer , & mut s) ? ; Ok (s) }
};
}
