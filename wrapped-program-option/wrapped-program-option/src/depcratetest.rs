// Generated macro for test (module)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_from_rust_option () { let option = Some (99u64) ; let c_option : COption < u64 > = option . into () ; assert_eq ! (c_option , COption :: Some (99u64)) ; let expected = c_option . into () ; assert_eq ! (option , expected) ; let option = None ; let c_option : COption < u64 > = option . into () ; assert_eq ! (c_option , COption :: None) ; let expected = c_option . into () ; assert_eq ! (option , expected) ; } }
};
}
