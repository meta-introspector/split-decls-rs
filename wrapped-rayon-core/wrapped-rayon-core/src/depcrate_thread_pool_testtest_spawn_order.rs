// Generated macro for test_spawn_order (macro)
macro_rules! Depcrate_thread_pool_testtest_spawn_order {
() => {
// Module: crate::thread_pool::test
// Provides: {"test_spawn_order"}
// Dependencies: {}
macro_rules ! test_spawn_order { ($ spawn : ident) => { { let builder = ThreadPoolBuilder :: new () . num_threads (1) ; let pool = & builder . build () . unwrap () ; let (tx , rx) = channel () ; pool . install (move || { for i in 0 .. 10 { let tx = tx . clone () ; pool .$ spawn (move || { tx . send (i) . unwrap () ; }) ; } }) ; rx . iter () . collect ::< Vec < i32 >> () } } ; }
};
}
