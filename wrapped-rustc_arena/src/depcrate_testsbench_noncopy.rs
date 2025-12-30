// Generated macro for bench_noncopy (function)
macro_rules! Depcrate_testsbench_noncopy {
() => {
// Module: crate::tests
// Provides: {"bench_noncopy"}
// Dependencies: {}
# [bench] fn bench_noncopy (b : & mut Bencher) { let arena = TypedArena :: default () ; b . iter (| | { arena . alloc (Noncopy { string : "hello world" . to_string () , array : vec ! [1 , 2 , 3 , 4 , 5] }) }) }
};
}
