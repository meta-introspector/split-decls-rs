// Generated macro for default_is_random (function)
macro_rules! Depcrate_tests_uuiddefault_is_random {
() => {
// Module: crate::tests::uuid
// Provides: {"default_is_random"}
// Dependencies: {}
# [test] fn default_is_random () { let uuid1 = < Retained < NSUUID > > :: default () ; let uuid2 = NSUUID :: UUID () ; assert_ne ! (uuid1 , uuid2 , "Statistically very unlikely") ; }
};
}
