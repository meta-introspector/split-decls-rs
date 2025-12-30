// Generated macro for test (function)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [test] # [expect (clippy :: field_reassign_with_default)] fn test () -> Result < () > { let mut container = RefWithFloat :: default () ; container . Value = Some (PropertyValue :: CreateSingle (1.23) ? . cast () ?) ; assert_eq ! (container . Value . unwrap () . Value () ?, 1.23) ; Ok (()) }
};
}
