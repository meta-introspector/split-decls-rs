// Generated macro for test_round_trip_non_ascii_internet_password (function)
macro_rules! Depcratetest_round_trip_non_ascii_internet_password {
() => {
// Module: crate
// Provides: {"test_round_trip_non_ascii_internet_password"}
// Dependencies: {}
fn test_round_trip_non_ascii_internet_password () { println ! ("test_round_trip_non_ascii_internet_password: start") ; let name = "test_round_trip_non_ascii_internet_password" ; let password = "このきれいな花は桜です" . as_bytes () ; set_internet_password (name , None , name , "/test" , None , HTTP , Any , password) . unwrap () ; let stored_password = get_internet_password (name , None , name , "/test" , None , HTTP , Any) . unwrap () ; assert_eq ! (stored_password , password) ; delete_internet_password (name , None , name , "/test" , None , HTTP , Any) . unwrap () ; println ! ("test_round_trip_non_ascii_internet_password: pass") ; }
};
}
