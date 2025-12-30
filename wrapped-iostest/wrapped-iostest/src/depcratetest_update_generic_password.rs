// Generated macro for test_update_generic_password (function)
macro_rules! Depcratetest_update_generic_password {
() => {
// Module: crate
// Provides: {"test_update_generic_password"}
// Dependencies: {}
fn test_update_generic_password () { println ! ("test_update_generic_password: start") ; let name = "test_update_generic_password" ; let password = b"test ascii password" . as_slice () ; set_generic_password (name , name , password) . unwrap () ; let stored_password = get_generic_password (name , name) . unwrap () ; assert_eq ! (stored_password , password) ; let password = "このきれいな花は桜です" . as_bytes () ; set_generic_password (name , name , password) . unwrap () ; let stored_password = get_generic_password (name , name) . unwrap () ; assert_eq ! (stored_password , password) ; delete_generic_password (name , name) . unwrap () ; println ! ("test_update_generic_password: pass") ; }
};
}
