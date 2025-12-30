// Generated macro for tests (module)
macro_rules! Depcrate_integertests {
() => {
// Module: crate::integer
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Integer ; # [test] fn from_str_limits () { assert_eq ! (Integer :: from_str ("-1") , Ok ((- 1) . into ())) ; assert_eq ! (Integer :: from_str ("0") , Ok (0 . into ())) ; assert_eq ! (Integer :: from_str ("1") , Ok (1 . into ())) ; assert_eq ! (Integer :: from_str ("-9223372036854775808") , Ok ((- 9223372036854775808i64) . into ())) ; assert ! (Integer :: from_str ("-9223372036854775809") . is_err ()) ; assert_eq ! (Integer :: from_str ("18446744073709551615") , Ok (18446744073709551615u64 . into ())) ; assert ! (Integer :: from_str ("18446744073709551616") . is_err ()) ; } }
};
}
