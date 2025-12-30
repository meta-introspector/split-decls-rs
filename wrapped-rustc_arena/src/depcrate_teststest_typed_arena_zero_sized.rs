// Generated macro for test_typed_arena_zero_sized (function)
macro_rules! Depcrate_teststest_typed_arena_zero_sized {
() => {
// Module: crate::tests
// Provides: {"test_typed_arena_zero_sized"}
// Dependencies: {}
# [test] fn test_typed_arena_zero_sized () { let arena = TypedArena :: default () ; # [cfg (not (miri))] const N : usize = 100000 ; # [cfg (miri)] const N : usize = 1000 ; for _ in 0 .. N { arena . alloc (()) ; } }
};
}
