// Generated macro for scope_spawn_broadcast (function)
macro_rules! Depcrate_scope_testscope_spawn_broadcast {
() => {
// Module: crate::scope::test
// Provides: {"scope_spawn_broadcast"}
// Dependencies: {}
# [test] fn scope_spawn_broadcast () { let sum = AtomicUsize :: new (0) ; let n = scope (| s | { s . spawn_broadcast (| _ , ctx | { sum . fetch_add (ctx . index () , Ordering :: Relaxed) ; }) ; crate :: current_num_threads () }) ; assert_eq ! (sum . into_inner () , n * (n - 1) / 2) ; }
};
}
