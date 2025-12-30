// Generated macro for tests (module)
macro_rules! Depcrate_tz_posixtests {
() => {
// Module: crate::tz::posix
// Provides: {"tests"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [cfg (test)] mod tests { use super :: * ; # [cfg (feature = "tz-system")] # [test] fn parse_posix_tz () { assert ! (PosixTzEnv :: parse ("EST5EDT") . is_err ()) ; let tz = PosixTzEnv :: parse (":EST5EDT") . unwrap () ; assert_eq ! (tz , PosixTzEnv :: Implementation ("EST5EDT" . into ())) ; assert ! (PosixTzEnv :: parse (b":EST5\xFFEDT") . is_err ()) ; } }
};
}
