// Generated macro for bench_copy (function)
macro_rules! Depcrate_testsbench_copy {
() => {
// Module: crate::tests
// Provides: {"bench_copy"}
// Dependencies: {}
# [bench] fn bench_copy (b : & mut Bencher) { let arena = TypedArena :: default () ; b . iter (| | arena . alloc (Point { x : 1 , y : 2 , z : 3 })) }
};
}
