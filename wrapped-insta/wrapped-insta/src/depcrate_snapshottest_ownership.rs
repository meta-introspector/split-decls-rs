// Generated macro for test_ownership (function)
macro_rules! Depcrate_snapshottest_ownership {
() => {
// Module: crate::snapshot
// Provides: {"test_ownership"}
// Dependencies: {}
# [doc = " Check that snapshots don't take ownership of the value"] # [test] fn test_ownership () { use std :: ops :: Range ; let r = Range { start : 0 , end : 10 } ; assert_debug_snapshot ! (r , @ "0..10") ; assert_debug_snapshot ! (r , @ "0..10") ; }
};
}
