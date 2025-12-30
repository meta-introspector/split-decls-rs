// Generated macro for mutual_install (function)
macro_rules! Depcrate_thread_pool_testmutual_install {
() => {
// Module: crate::thread_pool::test
// Provides: {"mutual_install"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn mutual_install () { let pool1 = ThreadPoolBuilder :: new () . num_threads (1) . build () . unwrap () ; let pool2 = ThreadPoolBuilder :: new () . num_threads (1) . build () . unwrap () ; let ok = pool1 . install (| | { pool2 . install (| | { pool1 . install (| | { true }) }) }) ; assert ! (ok) ; }
};
}
