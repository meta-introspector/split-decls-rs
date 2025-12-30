// Generated macro for bench_noncopy_nonarena (function)
macro_rules! Depcrate_testsbench_noncopy_nonarena {
() => {
// Module: crate::tests
// Provides: {"bench_noncopy_nonarena"}
// Dependencies: {}
# [bench] fn bench_noncopy_nonarena (b : & mut Bencher) { b . iter (| | { let _ : Box < _ > = Box :: new (Noncopy { string : "hello world" . to_string () , array : vec ! [1 , 2 , 3 , 4 , 5] }) ; }) }
};
}
