// Generated macro for test_missing_generic_password (function)
macro_rules! Depcratetest_missing_generic_password {
() => {
// Module: crate
// Provides: {"test_missing_generic_password"}
// Dependencies: {}
fn test_missing_generic_password () { println ! ("test_missing_generic_password: start") ; let name = "test_missing_generic_password" ; let result = delete_generic_password (name , name) ; match result { Ok (()) => () , Err (err) if err . code () == errSecItemNotFound => () , Err (err) => panic ! ("test_missing_generic_password: delete failed with status: {}" , err . code ()) , } let result = get_generic_password (name , name) ; match result { Ok (bytes) => panic ! ("test_missing_password: get returned {bytes:?}") , Err (err) if err . code () == errSecItemNotFound => () , Err (err) => panic ! ("test_missing_generic_password: get failed with status: {}" , err . code ()) , } let result = delete_generic_password (name , name) ; match result { Ok (()) => panic ! ("test_missing_generic_password: second delete found a password") , Err (err) if err . code () == errSecItemNotFound => () , Err (err) => panic ! ("test_missing_generic_password: delete failed with status: {}" , err . code ()) , } println ! ("test_missing_generic_password: pass") ; }
};
}
