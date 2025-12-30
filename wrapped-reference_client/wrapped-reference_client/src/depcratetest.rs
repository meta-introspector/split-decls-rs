// Generated macro for test (function)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [test] fn test () -> Result < () > { let reference = Reference :: new () ? ; assert_eq ! (reference . ToString () ?, "Reference") ; assert_eq ! (reference . Method (& reference) ?, "Reference") ; Ok (()) }
};
}
