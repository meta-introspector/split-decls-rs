// Generated macro for test_order (macro)
macro_rules! Depcrate_spawn_testtest_order {
() => {
// Module: crate::spawn::test
// Provides: {"test_order"}
// Dependencies: {}
macro_rules ! test_order { ($ outer_spawn : ident , $ inner_spawn : ident) => { { let builder = ThreadPoolBuilder :: new () . num_threads (1) ; let pool = builder . build () . unwrap () ; let (tx , rx) = channel () ; pool . install (move || { for i in 0 .. 10 { let tx = tx . clone () ; $ outer_spawn (move || { for j in 0 .. 10 { let tx = tx . clone () ; $ inner_spawn (move || { tx . send (i * 10 + j) . unwrap () ; }) ; } }) ; } }) ; rx . iter () . collect ::< Vec < i32 >> () } } ; }
};
}
