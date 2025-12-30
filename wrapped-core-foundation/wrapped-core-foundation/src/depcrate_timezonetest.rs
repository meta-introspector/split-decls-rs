// Generated macro for test (module)
macro_rules! Depcrate_timezonetest {
() => {
// Module: crate::timezone
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: CFTimeZone ; # [test] fn timezone_comparison () { let system = CFTimeZone :: system () ; let default = CFTimeZone :: default () ; assert_eq ! (system , default) ; } }
};
}
