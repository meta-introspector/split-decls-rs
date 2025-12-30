// Generated macro for test_round_trip_empty_internet_password (function)
macro_rules! Depcratetest_round_trip_empty_internet_password {
() => {
// Module: crate
// Provides: {"test_round_trip_empty_internet_password"}
// Dependencies: {}
fn test_round_trip_empty_internet_password () { println ! ("test_round_trip_empty_internet_password: start") ; let name = "test_empty_internet_password_input" ; let in_pass = b"" . as_slice () ; set_internet_password (name , None , name , "/test" , None , HTTP , Any , in_pass) . unwrap () ; let out_pass = get_internet_password (name , None , name , "/test" , None , HTTP , Any) . unwrap () ; assert_eq ! (in_pass , out_pass) ; delete_internet_password (name , None , name , "/test" , None , HTTP , Any) . unwrap () ; println ! ("test_round_trip_empty_internet_password: pass") ; }
};
}
