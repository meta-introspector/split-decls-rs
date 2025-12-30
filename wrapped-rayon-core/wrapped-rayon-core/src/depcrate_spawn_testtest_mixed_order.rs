// Generated macro for test_mixed_order (macro)
macro_rules! Depcrate_spawn_testtest_mixed_order {
() => {
// Module: crate::spawn::test
// Provides: {"test_mixed_order"}
// Dependencies: {}
# [doc = " Test mixed spawns pushing a series of numbers, interleaved such"] # [doc = " such that negative values are using the second kind of spawn."] macro_rules ! test_mixed_order { ($ pos_spawn : ident , $ neg_spawn : ident) => { { let builder = ThreadPoolBuilder :: new () . num_threads (1) ; let pool = builder . build () . unwrap () ; let (tx , rx) = channel () ; pool . install (move || { spawn_send ! ($ pos_spawn , tx , 0) ; spawn_send ! ($ neg_spawn , tx , - 1) ; spawn_send ! ($ pos_spawn , tx , 1) ; spawn_send ! ($ neg_spawn , tx , - 2) ; spawn_send ! ($ pos_spawn , tx , 2) ; spawn_send ! ($ neg_spawn , tx , - 3) ; spawn_send ! ($ pos_spawn , tx , 3) ; }) ; rx . iter () . collect ::< Vec < i32 >> () } } ; }
};
}
