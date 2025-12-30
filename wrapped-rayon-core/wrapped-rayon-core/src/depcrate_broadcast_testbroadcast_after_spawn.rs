// Generated macro for broadcast_after_spawn (function)
macro_rules! Depcrate_broadcast_testbroadcast_after_spawn {
() => {
// Module: crate::broadcast::test
// Provides: {"broadcast_after_spawn"}
// Dependencies: {}
# [test] fn broadcast_after_spawn () { let (tx , rx) = channel () ; crate :: registry :: in_worker (move | _ , _ | { crate :: spawn (move | | tx . send (22) . unwrap ()) ; }) ; crate :: broadcast (| _ | { }) ; assert_eq ! (22 , rx . try_recv () . unwrap ()) ; }
};
}
