// Generated macro for for_feature_gated_tests_only (module)
macro_rules! Depcratefor_feature_gated_tests_only {
() => {
// Module: crate
// Provides: {"for_feature_gated_tests_only"}
// Dependencies: {}
# [cfg (test)] mod for_feature_gated_tests_only { # [cfg (not (feature = "chrono"))] use chrono as _ ; # [cfg (not (feature = "jiff"))] use jiff as _ ; # [cfg (not (feature = "anyhow"))] use serial_test as _ ; }
};
}
