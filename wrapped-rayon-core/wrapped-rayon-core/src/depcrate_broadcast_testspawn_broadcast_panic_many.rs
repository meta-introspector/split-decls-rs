// Generated macro for spawn_broadcast_panic_many (function)
macro_rules! Depcrate_broadcast_testspawn_broadcast_panic_many {
() => {
// Module: crate::broadcast::test
// Provides: {"spawn_broadcast_panic_many"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn spawn_broadcast_panic_many () { let (tx , rx) = channel () ; let (panic_tx , panic_rx) = channel () ; let pool = ThreadPoolBuilder :: new () . num_threads (7) . panic_handler (move | e | panic_tx . send (e) . unwrap ()) . build () . unwrap () ; pool . spawn_broadcast (move | ctx | { tx . send (()) . unwrap () ; if ctx . index () % 2 == 0 { panic ! ("Hello, world!") ; } }) ; drop (pool) ; assert_eq ! (rx . into_iter () . count () , 7) ; assert_eq ! (panic_rx . into_iter () . count () , 4) ; }
};
}
