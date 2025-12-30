// Generated macro for out_of_order (function)
macro_rules! Depcrate_snapshot_map_testsout_of_order {
() => {
// Module: crate::snapshot_map::tests
// Provides: {"out_of_order"}
// Dependencies: {}
# [test] # [should_panic] fn out_of_order () { let mut map = SnapshotMap :: default () ; map . insert (22 , "twenty-two") ; let snapshot1 = map . snapshot () ; map . insert (33 , "thirty-three") ; let snapshot2 = map . snapshot () ; map . insert (44 , "forty-four") ; map . rollback_to (snapshot1) ; map . rollback_to (snapshot2) ; }
};
}
