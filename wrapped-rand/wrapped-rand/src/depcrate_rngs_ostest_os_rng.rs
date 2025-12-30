// Generated macro for test_os_rng (function)
macro_rules! Depcrate_rngs_ostest_os_rng {
() => {
// Module: crate::rngs::os
// Provides: {"test_os_rng"}
// Dependencies: {}
# [test] fn test_os_rng () { let x = OsRng . try_next_u64 () . unwrap () ; let y = OsRng . try_next_u64 () . unwrap () ; assert ! (x != 0) ; assert ! (x != y) ; }
};
}
