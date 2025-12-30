// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { is , Stream } ; # [test] # [cfg (windows)] fn is_err () { assert ! (! is (Stream :: Stderr)) } # [test] # [cfg (windows)] fn is_out () { assert ! (! is (Stream :: Stdout)) } # [test] # [cfg (windows)] fn is_in () { assert ! (is (Stream :: Stdin)) } # [test] # [cfg (unix)] fn is_err () { assert ! (is (Stream :: Stderr)) } # [test] # [cfg (unix)] fn is_out () { assert ! (is (Stream :: Stdout)) } # [test] # [cfg (target_os = "macos")] fn is_in () { assert ! (is (Stream :: Stdin)) } # [test] # [cfg (all (not (target_os = "macos") , unix))] fn is_in () { assert ! (is (Stream :: Stdin)) } }
};
}
