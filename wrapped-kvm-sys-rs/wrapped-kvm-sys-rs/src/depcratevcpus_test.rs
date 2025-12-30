// Generated macro for vcpus_test (function)
macro_rules! Depcratevcpus_test {
() => {
// Module: crate
// Provides: {"vcpus_test"}
// Dependencies: {}
# [test] fn vcpus_test () { let h = System :: initialize () . unwrap () ; assert ! (h . max_vcpus () >= h . recommended_vcpus ()) ; }
};
}
