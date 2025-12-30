// Generated macro for test_static_method (function)
macro_rules! Depcratetest_static_method {
() => {
// Module: crate
// Provides: {"test_static_method"}
// Dependencies: {}
# [test] fn test_static_method () { let c_str = unsafe { bindings :: Test :: name () } ; let name = unsafe { CStr :: from_ptr (c_str) . to_string_lossy () . into_owned () } ; assert_eq ! (name , "Test" , "Calling a static C++ method works!") ; }
};
}
