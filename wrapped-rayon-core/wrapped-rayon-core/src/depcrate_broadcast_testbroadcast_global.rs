// Generated macro for broadcast_global (function)
macro_rules! Depcrate_broadcast_testbroadcast_global {
() => {
// Module: crate::broadcast::test
// Provides: {"broadcast_global"}
// Dependencies: {}
# [test] fn broadcast_global () { let v = crate :: broadcast (| ctx | ctx . index ()) ; assert ! (v . into_iter () . eq (0 .. crate :: current_num_threads ())) ; }
};
}
