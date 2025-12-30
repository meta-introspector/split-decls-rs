// Generated macro for assert_deterministic_mtime (function)
macro_rules! Depcrateassert_deterministic_mtime {
() => {
// Module: crate
// Provides: {"assert_deterministic_mtime"}
// Dependencies: {}
# [track_caller] pub fn assert_deterministic_mtime (path : impl AsRef < Path >) { const DETERMINISTIC_TIMESTAMP : u64 = 1153704088 ; let path = path . as_ref () ; let mtime = path . metadata () . unwrap () . modified () . unwrap () ; let timestamp = mtime . duration_since (std :: time :: UNIX_EPOCH) . unwrap () . as_secs () ; assert_eq ! (timestamp , DETERMINISTIC_TIMESTAMP , "expected deterministic mtime for {path:?}, got {timestamp}") ; }
};
}
