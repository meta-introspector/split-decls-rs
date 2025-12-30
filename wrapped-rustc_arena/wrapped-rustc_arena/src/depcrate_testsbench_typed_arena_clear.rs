// Generated macro for bench_typed_arena_clear (function)
macro_rules! Depcrate_testsbench_typed_arena_clear {
() => {
// Module: crate::tests
// Provides: {"bench_typed_arena_clear"}
// Dependencies: {}
# [bench] fn bench_typed_arena_clear (b : & mut Bencher) { let mut arena = TypedArena :: default () ; b . iter (| | { arena . alloc (Point { x : 1 , y : 2 , z : 3 }) ; arena . clear () ; }) }
};
}
