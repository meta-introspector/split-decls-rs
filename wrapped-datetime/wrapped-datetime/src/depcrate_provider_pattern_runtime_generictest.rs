// Generated macro for test (module)
macro_rules! Depcrate_provider_pattern_runtime_generictest {
() => {
// Module: crate::provider::pattern::runtime::generic
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "datagen")] mod test { use super :: * ; # [test] fn test_runtime_generic_pattern_combine () { let pattern : GenericPattern = "{1} 'at' {0}" . parse () . expect ("Failed to parse a generic pattern.") ; let date = "y/M/d" . parse () . expect ("Failed to parse a date pattern.") ; let time = "HH:mm" . parse () . expect ("Failed to parse a time pattern.") ; let pattern = pattern . combined (date , time) . expect ("Failed to combine date and time.") ; assert_eq ! (pattern . to_string () , "y/M/d 'at' HH:mm") ; } }
};
}
