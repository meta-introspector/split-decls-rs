// Generated macro for test_missing_internet_password (function)
macro_rules! Depcratetest_missing_internet_password {
() => {
// Module: crate
// Provides: {"test_missing_internet_password"}
// Dependencies: {}
fn test_missing_internet_password () { println ! ("test_missing_internet_password: start") ; let name = "test_missing_internet_password" ; let result = delete_internet_password (name , None , name , "/test" , None , HTTP , Any) ; match result { Ok (()) => () , Err (err) if err . code () == errSecItemNotFound => () , Err (err) => panic ! ("test_missing_internet_password: delete failed with status: {}" , err . code ()) , } let result = get_internet_password (name , None , name , "/test" , None , HTTP , Any) ; match result { Ok (bytes) => panic ! ("test_missing_password: get returned {bytes:?}") , Err (err) if err . code () == errSecItemNotFound => () , Err (err) => panic ! ("test_missing_internet_password: get failed with status: {}" , err . code ()) , } let result = delete_internet_password (name , None , name , "/test" , None , HTTP , Any) ; match result { Ok (()) => panic ! ("test_missing_internet_password: second delete found a password") , Err (err) if err . code () == errSecItemNotFound => () , Err (err) => panic ! ("test_missing_internet_password: delete failed with status: {}" , err . code ()) , } println ! ("test_missing_internet_password: pass") ; }
};
}
