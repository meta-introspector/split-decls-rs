// Generated macro for test_round_trip_non_ascii_generic_password (function)
macro_rules! Depcratetest_round_trip_non_ascii_generic_password {
() => {
// Module: crate
// Provides: {"test_round_trip_non_ascii_generic_password"}
// Dependencies: {}
fn test_round_trip_non_ascii_generic_password () { println ! ("test_round_trip_non_ascii_generic_password: start") ; let name = "test_round_trip_non_ascii_generic_password" ; let password = "このきれいな花は桜です" . as_bytes () ; set_generic_password (name , name , password) . unwrap () ; let stored_password = get_generic_password (name , name) . unwrap () ; assert_eq ! (stored_password , password) ; delete_generic_password (name , name) . unwrap () ; println ! ("test_round_trip_non_ascii_generic_password: pass") ; }
};
}
