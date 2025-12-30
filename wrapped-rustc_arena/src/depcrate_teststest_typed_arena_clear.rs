// Generated macro for test_typed_arena_clear (function)
macro_rules! Depcrate_teststest_typed_arena_clear {
() => {
// Module: crate::tests
// Provides: {"test_typed_arena_clear"}
// Dependencies: {}
# [test] fn test_typed_arena_clear () { let mut arena = TypedArena :: default () ; for _ in 0 .. 10 { arena . clear () ; # [cfg (not (miri))] const N : usize = 10000 ; # [cfg (miri)] const N : usize = 100 ; for _ in 0 .. N { arena . alloc (Point { x : 1 , y : 2 , z : 3 }) ; } } }
};
}
