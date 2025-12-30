// Generated macro for check_capability_test (function)
macro_rules! Depcratecheck_capability_test {
() => {
// Module: crate
// Provides: {"check_capability_test"}
// Dependencies: {}
# [test] fn check_capability_test () { let h = System :: initialize () . unwrap () ; assert ! (h . check_capability (Capability :: UserMemory) != 0) ; }
};
}
