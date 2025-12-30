// Generated macro for test_static_array (function)
macro_rules! Depcratetest_static_array {
() => {
// Module: crate
// Provides: {"test_static_array"}
// Dependencies: {}
# [test] fn test_static_array () { let mut test = unsafe { bindings :: Test_COUNTDOWN . as_ptr () } ; let expected = unsafe { bindings :: Test_countdown () } ; let also_expected = unsafe { bindings :: Test_COUNTDOWN_PTR } ; assert ! (! test . is_null ()) ; assert_eq ! (also_expected , expected) ; assert_eq ! (test , also_expected) ; let mut expected = 10 ; unsafe { loop { assert_eq ! (* test , expected) ; if * test == 0 { break ; } test = test . offset (1) ; expected -= 1 ; } } }
};
}
