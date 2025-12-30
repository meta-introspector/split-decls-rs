// Generated macro for scope_spawn_broadcast_panic_many (function)
macro_rules! Depcrate_scope_testscope_spawn_broadcast_panic_many {
() => {
// Module: crate::scope::test
// Provides: {"scope_spawn_broadcast_panic_many"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn scope_spawn_broadcast_panic_many () { let count = AtomicUsize :: new (0) ; let pool = ThreadPoolBuilder :: new () . num_threads (7) . build () . unwrap () ; let result = crate :: unwind :: halt_unwinding (| | { pool . scope (| s | { s . spawn_broadcast (| _ , ctx | { count . fetch_add (1 , Ordering :: Relaxed) ; if ctx . index () % 2 == 0 { panic ! ("Hello, world!") ; } }) ; }) ; }) ; assert_eq ! (count . into_inner () , 7) ; assert ! (result . is_err () , "broadcast panic should propagate!") ; }
};
}
