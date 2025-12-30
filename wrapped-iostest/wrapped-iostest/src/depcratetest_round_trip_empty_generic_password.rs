// Generated macro for test_round_trip_empty_generic_password (function)
macro_rules! Depcratetest_round_trip_empty_generic_password {
() => {
// Module: crate
// Provides: {"test_round_trip_empty_generic_password"}
// Dependencies: {}
fn test_round_trip_empty_generic_password () { println ! ("test_round_trip_empty_generic_password: start") ; let name = "test_empty_generic_password_input" ; let in_pass = b"" ; set_generic_password (name , name , in_pass) . unwrap () ; let out_pass = get_generic_password (name , name) . unwrap () ; assert_eq ! (in_pass . as_slice () , out_pass) ; delete_generic_password (name , name) . unwrap () ; println ! ("test_round_trip_empty_generic_password: pass") ; }
};
}
