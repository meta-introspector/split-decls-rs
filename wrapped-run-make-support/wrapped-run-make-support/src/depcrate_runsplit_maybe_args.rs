// Generated macro for split_maybe_args (function)
macro_rules! Depcrate_runsplit_maybe_args {
() => {
// Module: crate::run
// Provides: {"split_maybe_args"}
// Dependencies: {}
fn split_maybe_args (s : & str) -> Vec < OsString > { s . split (' ') . filter_map (| s | { if s . chars () . all (| c | c . is_whitespace ()) { None } else { Some (OsString :: from (s)) } }) . collect () }
};
}
