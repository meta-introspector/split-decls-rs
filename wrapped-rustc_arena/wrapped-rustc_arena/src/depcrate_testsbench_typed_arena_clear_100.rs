// Generated macro for bench_typed_arena_clear_100 (function)
macro_rules! Depcrate_testsbench_typed_arena_clear_100 {
() => {
// Module: crate::tests
// Provides: {"bench_typed_arena_clear_100"}
// Dependencies: {}
# [bench] fn bench_typed_arena_clear_100 (b : & mut Bencher) { let mut arena = TypedArena :: default () ; b . iter (| | { for _ in 0 .. 100 { arena . alloc (Point { x : 1 , y : 2 , z : 3 }) ; } arena . clear () ; }) }
};
}
